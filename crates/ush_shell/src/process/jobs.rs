#[cfg(test)]
mod tests;

use std::process::{Child, Command, Stdio};

use anyhow::{Context, Result, anyhow, bail};

use crate::{Shell, ValueStream, repl::ReplJobCandidate, signal};

use super::render_alias_prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JobState {
    Running,
    Done(i32),
}

pub(crate) struct Job {
    id: usize,
    command: String,
    pid: u32,
    child: Child,
    state: JobState,
}

impl Shell {
    pub(crate) fn handle_jobs(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        if !args.is_empty() {
            bail!("jobs does not accept arguments");
        }
        self.refresh_jobs()?;
        let text = self
            .jobs
            .iter()
            .map(|job| format!("[{}] {:<7} {}\n", job.id, job.state.label(), job.command))
            .collect::<String>();
        Ok((ValueStream::Text(text), 0))
    }

    pub(crate) fn handle_wait(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        self.refresh_jobs()?;
        let job_ids = if args.is_empty() {
            self.jobs.iter().map(|job| job.id).collect::<Vec<_>>()
        } else {
            args.iter()
                .map(|arg| self.parse_job_spec(arg))
                .collect::<Result<Vec<_>>>()?
        };

        let mut last_status = 0;
        for id in job_ids {
            last_status = self.wait_for_job(id)?;
        }

        Ok((ValueStream::Empty, last_status))
    }

    pub(crate) fn handle_disown(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        self.refresh_jobs()?;
        let job_ids = if args.is_empty() {
            vec![self.current_job_id()?]
        } else {
            args.iter()
                .map(|arg| self.parse_job_spec(arg))
                .collect::<Result<Vec<_>>>()?
        };
        self.remove_jobs(&job_ids)?;
        Ok((ValueStream::Empty, 0))
    }

    pub(crate) fn handle_fg(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        self.refresh_jobs()?;
        let id = self.resolve_single_job_spec("fg", args)?;
        let position = self
            .job_position(id)
            .ok_or_else(|| anyhow!("unknown job: %{id}"))?;
        if self.jobs[position].state == JobState::Running
            && let Err(error) = signal::continue_background_job(self.jobs[position].pid)
            && !signal::is_missing_process(&error)
        {
            return Err(error).with_context(|| format!("failed to continue job %{id}"));
        }
        let status = self.wait_for_job(id)?;
        Ok((ValueStream::Empty, status))
    }

    pub(crate) fn handle_bg(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        self.refresh_jobs()?;
        let id = self.resolve_single_job_spec("bg", args)?;
        let position = self
            .job_position(id)
            .ok_or_else(|| anyhow!("unknown job: %{id}"))?;
        let job = &mut self.jobs[position];
        if job.state != JobState::Running {
            bail!("job %{id} is no longer running");
        }
        if let Err(error) = signal::continue_background_job(job.pid) {
            if signal::is_missing_process(&error) {
                bail!("job %{id} is no longer running");
            }
            return Err(error).with_context(|| format!("failed to continue job %{id}"));
        }
        job.state = JobState::Running;
        Ok((ValueStream::Empty, 0))
    }

    pub(crate) fn repl_job_candidates(&mut self) -> Vec<ReplJobCandidate> {
        let _ = self.refresh_jobs();
        self.jobs
            .iter()
            .map(|job| ReplJobCandidate {
                spec: format!("%{}", job.id),
                summary: format!("{}  {}", job.state.label(), job.command),
            })
            .collect()
    }

    pub(crate) fn spawn_background_job(&mut self, source: &str) -> Result<String> {
        let mut command = Command::new("/bin/sh");
        let mut script = render_alias_prelude(&self.aliases);
        script.push_str(source);
        command
            .arg("-c")
            .arg(script)
            .current_dir(&self.cwd)
            .envs(&self.env)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        signal::prepare_background_command(&mut command);

        let child = command
            .spawn()
            .with_context(|| format!("failed to spawn background job `{source}`"))?;
        let id = self.next_job_id;
        self.next_job_id += 1;
        let pid = child.id();
        self.jobs.push(Job {
            id,
            command: source.to_string(),
            pid,
            child,
            state: JobState::Running,
        });
        Ok(format!("[{id}] {pid}\n"))
    }

    fn refresh_jobs(&mut self) -> Result<()> {
        for job in &mut self.jobs {
            if job.state != JobState::Running {
                continue;
            }
            if let Some(status) = job.child.try_wait()? {
                job.state = JobState::Done(signal::exit_status(status));
            }
        }
        Ok(())
    }

    fn parse_job_spec(&self, value: &str) -> Result<usize> {
        let trimmed = value.strip_prefix('%').unwrap_or(value);
        let id = trimmed
            .parse::<usize>()
            .with_context(|| format!("invalid job spec: {value}"))?;
        if self.job_position(id).is_none() {
            bail!("unknown job: %{id}");
        }
        Ok(id)
    }

    fn resolve_single_job_spec(&self, command: &str, args: &[String]) -> Result<usize> {
        if args.len() > 1 {
            bail!("{command} accepts at most one job spec");
        }
        match args.first() {
            Some(arg) => self.parse_job_spec(arg),
            None => self.current_job_id(),
        }
    }

    fn current_job_id(&self) -> Result<usize> {
        self.jobs
            .last()
            .map(|job| job.id)
            .ok_or_else(|| anyhow!("no jobs"))
    }

    fn job_position(&self, id: usize) -> Option<usize> {
        self.jobs.iter().position(|job| job.id == id)
    }

    fn wait_for_job(&mut self, id: usize) -> Result<i32> {
        let position = self
            .job_position(id)
            .ok_or_else(|| anyhow!("unknown job: %{id}"))?;
        let mut job = self.jobs.remove(position);
        let status = match job.state {
            JobState::Running => signal::exit_status(job.child.wait()?),
            JobState::Done(status) => status,
        };
        Ok(status)
    }

    fn remove_jobs(&mut self, ids: &[usize]) -> Result<()> {
        let mut positions = ids
            .iter()
            .map(|id| {
                self.job_position(*id)
                    .ok_or_else(|| anyhow!("unknown job: %{id}"))
            })
            .collect::<Result<Vec<_>>>()?;
        positions.sort_unstable();
        positions.dedup();
        for position in positions.into_iter().rev() {
            self.jobs.remove(position);
        }
        Ok(())
    }
}

impl JobState {
    fn label(self) -> &'static str {
        match self {
            JobState::Running => "Running",
            JobState::Done(0) => "Done",
            JobState::Done(_) => "Exit",
        }
    }
}
