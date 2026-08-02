🏃‍♂️ Pacer: Enable auto-merge for Jules bot PRs

💡 What: Created a new GitHub Actions workflow (`.github/workflows/auto-merge.yml`) that automatically enables auto-merge for pull requests created by `google-labs-jules[bot]`.
🎯 Why: To streamline the development process by allowing automated bot PRs to merge automatically once all required Continuous Integration (CI) checks pass, reducing manual intervention.
📊 Impact: Faster turnaround time for automated changes and updates.
🔬 Measurement: Verify that when `google-labs-jules[bot]` opens a PR and the CI workflow (`ci.yml`) succeeds, the PR is automatically merged into `main`.
