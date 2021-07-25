FROM mcr.microsoft.com/dotnet/sdk:5.0 AS installer-env
# Build requires 3.1 SDK too
COPY --from=mcr.microsoft.com/dotnet/sdk:3.1 /usr/share/dotnet /usr/share/dotnet

ARG CRON_CLEANUP_SCHEDULE
ENV CRON_CLEANUP_SCHEDULE $CRON_CLEANUP_SCHEDULE

WORKDIR /src/dotnet-function-app
COPY HighScoreApi.fsproj .
RUN dotnet restore *.fsproj
COPY . .
RUN dotnet publish *.fsproj --output /home/site/wwwroot

FROM mcr.microsoft.com/azure-functions/dotnet-isolated:3.0-dotnet-isolated5.0
ENV AzureWebJobsScriptRoot=/home/site/wwwroot
ENV AzureFunctionsJobHost__Logging__Console__IsEnabled=true
COPY --from=installer-env ["/home/site/wwwroot", "/home/site/wwwroot"]
