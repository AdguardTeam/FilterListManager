using AdGuard.Utils.Base.Logging;
using AdGuard.Utils.Base.Logging.TraceListeners;
using NUnit.Framework;
using NUnit.Framework.Internal;

namespace AdGuard.FilterListManager.Test
{
	/// <summary>
	/// Implementation of the <see cref="ITraceListener"/> interface, that
	/// allows to write log messages to the test output.
	/// </summary>
	/// <seealso cref="ITraceListener" />
	public class TestContextTraceListener : ITraceListener
	{
		private readonly TestExecutionContext m_ExecutionContext;

		/// <summary>
		/// Initializes a new instance of the <see cref="TestContextTraceListener"/> class.
		/// </summary>
		/// <param name="executionContext">The execution context.</param>
		public TestContextTraceListener(TestExecutionContext executionContext)
		{
			m_ExecutionContext = executionContext;
		}

		/// <summary>
		/// Writes the line.
		/// </summary>
		/// <param name="message">The message.</param>
		public virtual void WriteLine(string message)
		{
			if (m_ExecutionContext.ExecutionStatus == TestExecutionStatus.Running)
			{
				return;
			}

			TestContext.WriteLine(message);
		}

		/// <summary>
		/// Flushes this log.
		/// </summary>
		public virtual void Flush()
		{
			if (m_ExecutionContext.ExecutionStatus == TestExecutionStatus.Running)
			{
				return;
			}

			TestContext.WriteLine("Flush method is called");
		}

		/// <summary>
		/// Level of the logging
		/// </summary>
		public LogLevel Level { get; set; }
	}
}
