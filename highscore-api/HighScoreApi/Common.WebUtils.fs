namespace HighScoreApi.Common

open System.Net
open Microsoft.Azure.Functions.Worker.Http

module WebUtils =

    /// Create response with CORS header set to allow all origins.
    /// Work-around for bad CORS support in Azure functions docker containers
    let resWithOkCors (status: HttpStatusCode) (req: HttpRequestData) : HttpResponseData =
        let res = req.CreateResponse(status)
        res.Headers.Add("Access-Control-Allow-Origin", "*")
        res

    let okResWithOkCors : HttpRequestData -> HttpResponseData = resWithOkCors HttpStatusCode.OK
    let badReqWithOkCors : HttpRequestData -> HttpResponseData = resWithOkCors HttpStatusCode.BadRequest
