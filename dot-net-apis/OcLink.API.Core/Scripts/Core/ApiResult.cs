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
