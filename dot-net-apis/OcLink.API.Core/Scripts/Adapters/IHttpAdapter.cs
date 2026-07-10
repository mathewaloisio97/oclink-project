using System.Collections.Generic;
using System.Threading.Tasks;

namespace OcLink.API.Core.Scripts.Adapters
{
    /// <summary>
    /// Defines an agnostic interface for HTTP operations to support dependency injection.
    /// </summary>
    /// <author>Mathew Aloisio</author>
    public interface IHttpAdapter
    {
        /// <summary>
        /// Executes an asynchronous HTTP POST request against the specified URL endpoint.
        /// </summary>
        /// <param name="pUrl">The absolute URL target.</param>
        /// <param name="pJsonBody">The serialized JSON body payload.</param>
        /// <param name="pBearerToken">The optional bearer token for zero-trust authorization.</param>
        /// <param name="pCustomHeaders">Optional dictionary of custom headers to append.</param>
        /// <returns>A standardized representation of the HTTP response.</returns>
        Task<HttpResponse> PostAsync(string pUrl, string pJsonBody, string pBearerToken = null, Dictionary<string, string> pCustomHeaders = null);

        /// <summary>
        /// Executes an asynchronous HTTP GET request against the specified URL endpoint.
        /// </summary>
        /// <param name="pUrl">The absolute URL target.</param>
        /// <param name="pBearerToken">The optional bearer token for zero-trust authorization.</param>
        /// <param name="pCustomHeaders">Optional dictionary of custom headers to append.</param>
        /// <returns>A standardized representation of the HTTP response.</returns>
        Task<HttpResponse> GetAsync(string pUrl, string pBearerToken = null, Dictionary<string, string> pCustomHeaders = null);
    }
}
