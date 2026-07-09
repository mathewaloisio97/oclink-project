/*
 * GNU AFFERO GENERAL PUBLIC LICENSE
 * Version 3, 19 November 2007
 *
 * Copyright (C) 2026 Mathew Aloisio
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

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
