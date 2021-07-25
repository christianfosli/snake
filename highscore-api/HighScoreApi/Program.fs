namespace HighScoreApi

open System.Text.Json
open Microsoft.Extensions.Configuration
open Microsoft.Extensions.DependencyInjection
open Microsoft.Extensions.Hosting

module App =

    [<EntryPoint>]
    let main _ =
        let builder =
            Host
                .CreateDefaultBuilder()
                .ConfigureFunctionsWorkerDefaults(fun builder ->
                    builder.Services.Configure<JsonSerializerOptions>
                        (fun (options: JsonSerializerOptions) ->
                            options.PropertyNamingPolicy <- JsonNamingPolicy.CamelCase)
                    |> ignore)
                .ConfigureAppConfiguration(fun builder -> builder.AddEnvironmentVariables() |> ignore)

        let host = builder.Build()

        async {
            do! Async.SwitchToThreadPool()
            do! host.RunAsync() |> Async.AwaitTask
        }
        |> Async.RunSynchronously

        0
