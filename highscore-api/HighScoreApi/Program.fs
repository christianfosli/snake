namespace HighScoreApi

open Microsoft.Extensions.Configuration
open Microsoft.Extensions.DependencyInjection
open Microsoft.Extensions.Hosting

module App =

    [<EntryPoint>]
    let main _ =
        let builder =
            Host
                .CreateDefaultBuilder()
                .ConfigureFunctionsWorkerDefaults()
                .ConfigureAppConfiguration(fun builder -> builder.AddEnvironmentVariables() |> ignore)

        let host = builder.Build()

        async {
            do! Async.SwitchToThreadPool()
            do! host.RunAsync() |> Async.AwaitTask
        }
        |> Async.RunSynchronously

        0
