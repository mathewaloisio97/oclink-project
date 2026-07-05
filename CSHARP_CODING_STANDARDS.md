# C# Coding Standards & Architecture Specification

This document defines the strict architectural and styling constraints for all C# code within the **OcLink** ecosystem. Adherence to these standards ensures our codebase remains high-performance, maintainable, and seamlessly compatible across standard `.NET` runtimes and Unity environments.

---

## Environmental & Structural Foundation

### Target & Runtime Compatibility
* **Language Specification:** Must be strictly **.NET Standard 2.0** compliant.
* **Runtime Targets:** Code must compile cleanly and run optimally across **Mono**, **IL2CPP**, and **CoreCLR** environments without modification.

### File Layout Restrictions
* **Single Responsibility Structure:** Maintain exactly **one class per file**. 
* **Unity Derivations:** This rule is absolute, especially for components inheriting from engine types like `MonoBehaviour`.

---

## Portability & Architectural Rules

### Engine Dependency Isolation (The Sandbox Rule)
* **Zero Engine Dependencies:** Projects within the `dot-net-apis/` directory must **never reference Unity assembly DLLs** (e.g., `UnityEngine.dll`, `UnityEditor.dll`, `Unity.Mathematics.dll`).
* **Pure .NET Execution:** The API layer must remain a pure C# state machine. If data primitives like positions, colors, or logs are needed, utilize standard .NET primitives or custom contract models, and map them to Unity-specific engine types *outside* the DLL boundary using the adapter pattern.

### The Compilation Guardrail
* **No Preprocessor Directives:** **Never use `#if` directives** (such as `#if UNITY_EDITOR` or `#if UNITY_ANDROID`) within this workspace. The codebase must be completely decoupled so it can optionally compile cleanly into a standalone managed DLL at any time.
* **The Adapter Pattern Mandate:** Any logic that requires platform-specific behavior or engine-locked dependencies must be abstracted out using **Adapter Patterns** or Interfaces. The execution layer can then be safely injected from outside the managed DLL boundaries.

### Event & Data Architecture

#### âEngine/Unity Contexts
When exposing event architecture to the Unity Editor that require argument passing, always derive a custom typed class, apply the `[Serializable]` attribute, and ensure every custom event type resides in its own dedicated file.

> **Unity Example:**
> ```csharp
> [Serializable] 
> public class ChallengeCompletedUnityEvent : UnityEvent<int> {}
> ```

#### âPure .NET Sandbox Contexts
Because `UnityEngine.dll` is blocked in the sandbox, use native C# `System.EventHandler<TEventArgs>`, `System.Action<T>`, or custom strongly-typed delegates instead of `UnityEvent`.

> **Pure .NET Example:**
> ```csharp
> // Resides in its own file
> public class ChallengeCompletedEventArgs : EventArgs
> {
>     public int ChallengeId { get; set; }
> }
> 
> // Inside your pure .NET service:
> public event EventHandler<ChallengeCompletedEventArgs> ChallengeCompleted;
> ```

---

## Coding Conventions & Naming Nomenclature

### Formatting & Syntax
* **Implicit Access Modifiers:** Do not explicitly use the `private` keyword where it is already language-implicit.
* **Optimized Increment Loops:** Prefer pre-increment (`++i`) over post-increment (`i++`) in scenarios where the final assignment outcome remains unchanged.
* **Explicit Bracing:** Never use unbraced loops. 
  * *Correct:* `for (int i = 0; i < count; ++i) { doFunc(); }`
  * *Incorrect:* `for (int i = 0; i < count; ++i) doFunc();`
* **Single-Line Conditional Constraints:** For 1-liner `if` statements, braces are optional, but the conditional body **must always be indented on the following line**. Never write the body inline.
  * *Correct:*
    ```csharp
    if (condition)
        ExecuteTargetAction();
    ```
  * *Incorrect:* `if (condition) ExecuteTargetAction();`
  * *Correct:*
    ```csharp
    if (condition)
        return;
    ```
  * *Incorrect:* `if (condition) return;`

### Prefix & Variable Conventions
* **Method Arguments:** All argument parameter names must carry a lowercase `p` prefix. (e.g., `public void ProcessData(int pArgOne, int pArgTwo)`).
* **Private Fields:** All private member variables must carry an `m_` prefix. (e.g., `m_MyFieldOne`).
* **Banned Vocabulary:** Never use the word `Unity` or any engine-banned keywords in class names or file names within the sandbox workspace (`dot-net-apis/`). The only exception is the `UnityEvent` class name suffix *strictly* within engine-dependent adapter boundaries (`unity-engine-apis/`).
* **Namespace Organization:** Design deeply structured sub-namespaces to encapsulate domain contexts and prevent root namespace bloat.

---

## Code Organization Layout (`#region`)

Developers must visually segment source files using clear `#region` markers to keep code scannable. Recommended categories include:

* `#region Editor Serialized Field(s)` *(Engine Layers Only)*
* `#region Public Properties`
* `#region Unity Callback(s)` *(Engine Layers Only)*
* `#region Public [Category Name] Method(s)`
* `#region Private Utility Method(s)`

---

## Documentation & Metadata Rules

### Metadata Layout & Equivalencies

To keep contextual documentation consistent between visual engine tools and pure backend code, metadata must map cleanly using the following layout pairs:

#### Engine / Unity Contexts
Fields exposed to the Inspector must use engine metadata markers for visual grouping and documentation:
```csharp
[Header("Networking Configuration")]
[Tooltip("The time interval in seconds between heartbeat network pings.")]
float m_PingInterval; // Implicit private
```

#### Pure .NET Sandbox Contexts (`dot-net-apis/`)
Since engine attributes will not compile in the sandbox, use **XML Comment Elements** (`<remarks>` as the structural equivalent to `[Header]`, and `<value>` or `<summary>` as the equivalent to `[Tooltip]`):
```csharp
/// <summary>
/// Managed ping utility interval mapping.
/// </summary>
/// <remarks>Category Context: Networking Configuration</remarks>
/// <value>The time interval in seconds between heartbeat network pings.</value>
float m_PingInterval; // Implicit private
```

### XML Document Standards
All classes across both codebases must feature thorough, enterprise-grade XML heading blocks following this exact presentation signature:

```csharp
/// <summary>
/// This is an example of an above a class XML summary.
/// This is another line in it providing comprehensive architectural context.
/// </summary>
/// <author>Author Name</author>
public class MyClass
{
    // Implementation conforms strictly to constraints above
}
```