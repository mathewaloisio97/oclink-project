namespace OcLink.API.Core.Scripts.Adapters
{
    /// <summary>
    /// Encapsulates the agnostic result of an HTTP operation across various 
    /// potential client implementations.
    /// </summary>
    /// <author>Mathew Aloisio</author>
    public class HttpResponse
    {
        /// <summary>
        /// The standard HTTP protocol status code.
        /// </summary>
        public int StatusCode { get; set; }

        /// <summary>
        /// The raw string content returned in the response body.
        /// </summary>
        public string Content { get; set; }

        /// <summary>
        /// Evaluates whether the status code represents a standardized success condition.
        /// </summary>
        public bool IsSuccessStatusCode => StatusCode >= 200 && StatusCode <= 299;
    }
}
