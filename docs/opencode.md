# Opencode Cost Summary & Monitoring

This document describes how to monitor and summarize the token usage and estimated execution costs of running OpenCode in GitHub Actions or locally. For more details on the workflow configuration, refer to [hello.yml](../.github/workflows/hello.yml).

## Official OpenCode Documentation & Resources

- **Official Website:** [opencode.ai](https://opencode.ai)
- **GitHub Repository:** [anomalyco/opencode](https://github.com/anomalyco/opencode)
- **Documentation:** [opencode.ai/docs](https://opencode.ai/docs)
- **Built-in Agents:**
  - **build**: Full-access agent for development work.
  - **plan**: Read-only agent for analysis and code exploration.

## GitHub Actions Cost Summary

The `.github/workflows/hello.yml` workflow automatically computes and appends an execution summary to the GitHub Actions job summary page (`$GITHUB_STEP_SUMMARY`). 

The summary includes:
- The specific model used for the run.
- Token usage metrics (Prompt Tokens, Completion Tokens, Total Tokens).
- Estimated monetary cost calculated dynamically based on public pricing tiers for the selected model.

## Local Cost Monitoring

When running OpenCode locally, you can track token usage and cost metrics directly from the CLI logs or output summaries provided after each task completion.
