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
