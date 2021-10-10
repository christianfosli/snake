namespace HighScoreApi

open Microsoft.Azure.WebJobs.Extensions.OpenApi.Core.Configurations
open Microsoft.Azure.WebJobs.Extensions.OpenApi.Core.Enums
open Microsoft.OpenApi.Models

type OpenApiConfig() =
    inherit DefaultOpenApiConfigurationOptions()

    override opt.OpenApiVersion = OpenApiVersionType.V3
