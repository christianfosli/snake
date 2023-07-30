# End-to-End tests

End to end tests for playwright

## Prereq

* NPM / Node

* Install browsers needed for testing:

  ```bash
  npx playwright install
  ```

## Running tests

```bash
# Run headless against prod
npx playwright test
# Run headless against localhost (run app with docker-compose first :wink:)
URL=localhost:8080 npx playwright test
# Open UI for running interactively
npx playwright test --ui
```
