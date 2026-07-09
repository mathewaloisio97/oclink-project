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
    /// Defines an interface for JSON serialization and deserialization allowing pluggable engine compatibility.
    /// </summary>
    /// <author>Mathew Aloisio</author>
    public interface IJsonAdapter
    {
        /// <summary>
        /// Serializes a managed object graph into its JSON string representation.
        /// </summary>
        /// <typeparam name="T">The type of the object to serialize.</typeparam>
        /// <param name="pObject">The target object to serialize.</param>
        /// <returns>A serialized JSON string.</returns>
        string Serialize<T>(T pObject);

        /// <summary>
        /// Deserializes a raw JSON string into a structured managed object.
        /// </summary>
        /// <typeparam name="T">The target structural schema.</typeparam>
        /// <param name="pJson">The raw JSON string payload.</param>
        /// <returns>The fully deserialized object instance.</returns>
        T Deserialize<T>(string pJson);
    }
}
