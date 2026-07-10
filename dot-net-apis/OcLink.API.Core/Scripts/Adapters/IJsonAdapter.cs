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
