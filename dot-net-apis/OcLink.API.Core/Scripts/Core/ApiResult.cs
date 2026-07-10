namespace OcLink.API.Core.Scripts.Core
{
    /// <summary>
    /// Represents the generic outcome of an API boundary operation.
    /// </summary>
    /// <typeparam name="T">The specific expected response model on success.</typeparam>
    /// <author>Mathew Aloisio</author>
    public class ApiResult<T>
    {
        /// <summary>
        /// Indicates if the operation completed successfully without server or network faults.
        /// </summary>
        public bool IsSuccess { get; set; }

        /// <summary>
        /// The underlying HTTP status code returned by the operation.
        /// </summary>
        public int StatusCode { get; set; }

        /// <summary>
        /// The deserialized data payload if the operation succeeded.
        /// </summary>
        public T Data { get; set; }

        /// <summary>
        /// The error message if the operation failed.
        /// </summary>
        public string ErrorMessage { get; set; }
    }
}
