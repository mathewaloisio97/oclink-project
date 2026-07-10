using System;
using OcLink.API.Core.Scripts.Adapters;

namespace OcLink.API.Core.Scripts.Core
{
    /// <summary>
    /// Provides a foundational base class containing shared dependency parsing for API boundaries.
    /// </summary>
    /// <author>Mathew Aloisio</author>
    public abstract class ApiClientBase
    {
        #region Protected Field(s)
        /// <summary>
        /// The injected transport layer adapter.
        /// </summary>
        protected readonly IHttpAdapter m_HttpAdapter;

        /// <summary>
        /// The injected serialization engine adapter.
        /// </summary>
        protected readonly IJsonAdapter m_JsonAdapter;

        /// <summary>
        /// The root gateway host endpoint.
        /// </summary>
        protected readonly string m_BaseUrl;
        #endregion

        #region Public Constructor(s)
        /// <summary>
        /// Instantiates the foundational dependencies.
        /// </summary>
        /// <param name="pHttpAdapter">The transport adapter for network boundaries.</param>
        /// <param name="pJsonAdapter">The formatting adapter for JSON manipulation.</param>
        /// <param name="pBaseUrl">The root URL spanning to the edge gateway host.</param>
        protected ApiClientBase(IHttpAdapter pHttpAdapter, IJsonAdapter pJsonAdapter, string pBaseUrl)
        {
            if (pHttpAdapter == null)
                throw new ArgumentNullException(nameof(pHttpAdapter));
            if (pJsonAdapter == null)
                throw new ArgumentNullException(nameof(pJsonAdapter));

            m_HttpAdapter = pHttpAdapter;
            m_JsonAdapter = pJsonAdapter;
            m_BaseUrl = pBaseUrl.TrimEnd('/');
        }
        #endregion

        #region Protected Utility Method(s)
        /// <summary>
        /// Standardizes agnostic HTTP responses into predictable type-safe API Result structures.
        /// </summary>
        /// <param name="pResponse">The raw HTTP response from the adapter.</param>
        /// <returns>A strongly typed ApiResult wrapping the expected payload.</returns>
        protected ApiResult<T> ProcessResponse<T>(HttpResponse pResponse)
        {
            ApiResult<T> result = new ApiResult<T> { StatusCode = pResponse.StatusCode };

            if (pResponse.IsSuccessStatusCode)
            {
                result.IsSuccess = true;
                try
                {
                    if (!string.IsNullOrEmpty(pResponse.Content))
                        result.Data = m_JsonAdapter.Deserialize<T>(pResponse.Content);
                }
                catch (Exception ex)
                {
                    result.IsSuccess = false;
                    result.ErrorMessage = $"Failed to parse successful response payload: {ex.Message}";
                }
            }
            else
            {
                result.IsSuccess = false;
                result.ErrorMessage = $"HTTP Error {pResponse.StatusCode}: {pResponse.Content}";
            }

            return result;
        }
        #endregion
    }
}
