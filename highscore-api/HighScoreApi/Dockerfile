FROM mcr.microsoft.com/dotnet/sdk:6.0 AS installer-env

ARG CRON_CLEANUP_SCHEDULE
ENV CRON_CLEANUP_SCHEDULE $CRON_CLEANUP_SCHEDULE

WORKDIR /src/dotnet-function-app
COPY HighScoreApi.fsproj .
RUN dotnet restore *.fsproj
COPY . .
RUN dotnet publish *.fsproj --output /home/site/wwwroot

FROM mcr.microsoft.com/azure-functions/dotnet-isolated:4-dotnet-isolated6.0
ENV AzureWebJobsScriptRoot=/home/site/wwwroot
ENV AzureFunctionsJobHost__Logging__Console__IsEnabled=true
COPY --from=installer-env ["/home/site/wwwroot", "/home/site/wwwroot"]
