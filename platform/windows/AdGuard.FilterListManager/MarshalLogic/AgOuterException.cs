namespace AdGuard.FilterListManager.MarshalLogic
{
    /// <summary>
    /// Main exception for communication with the main app.
    /// </summary>
    /// <seealso cref="UniffiException" />
    public class AgOuterException : UniffiException
    {
        AgOuterException(string message)
            : base(message) { }

        // Each variant is a nested class
        // Flat enums carries a string error message, so no special implementation is necessary.

        /// <summary>
        /// Occurs if the library is unable to open the filter DB.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class CannotOpenDatabaseException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.CannotOpenDatabaseException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public CannotOpenDatabaseException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if the file is not a filter DB.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class NotADatabaseException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.NotADatabaseException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public NotADatabaseException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if the disk is full.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class DiskFullException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.DiskFullException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public DiskFullException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if the DB is busy.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class DatabaseBusy : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.DiskFullException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public DatabaseBusy(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if the entity hasn't been found in the filter DB.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class EntityNotFoundException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.EntityNotFoundException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public EntityNotFoundException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if the library is unable to found the path given.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class PathNotFoundException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.PathNotFoundException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public PathNotFoundException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if the library is unable to access to the path given.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class PathHasDeniedPermissionException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.PathHasDeniedPermissionException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public PathHasDeniedPermissionException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs if path given is already exists.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class PathAlreadyExistsException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.PathAlreadyExistsException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public PathAlreadyExistsException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs when a timeout is off.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class TimedOutException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.TimedOutException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public TimedOutException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs when the library got an HTTP-related error.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class HttpClientNetworkException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.HttpClientNetworkException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public HttpClientNetworkException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs when the library is unable to parse the body of HTTP response.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class HttpClientBodyRecoveryFailedException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.HttpClientBodyRecoveryFailedException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public HttpClientBodyRecoveryFailedException(string message)
                : base(message) { }
        }
        
        /// <summary>
        /// For a few requests we strictly check response code 200. 204
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class HttpStrict200Response : AgOuterException
        {
            public HttpStrict200Response(string message) : base(message) { }
        }

        /// <summary>
        /// Downloaded filter body likely is not a filter. This might be a html page, for example
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class FilterContentIsLikelyNotAFilter : AgOuterException
        {
            public FilterContentIsLikelyNotAFilter(string message) : base(message) { }
        }

        /// <summary>
        /// Occurs when the library is unable to parse the filter given.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class FilterParserException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.FilterParserException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public FilterParserException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs when the field given is empty.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class FieldIsEmptyException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.FieldIsEmptyException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public FieldIsEmptyException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs on a mutex-related error.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class MutexException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.MutexException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public MutexException(string message)
                : base(message) { }
        }

        /// <summary>
        /// Occurs on other errors.
        /// </summary>
        /// <seealso cref="AgOuterException" />
        public class OtherException : AgOuterException
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="AgOuterException.OtherException"/> class.
            /// </summary>
            /// <param name="message">The message.</param>
            public OtherException(string message)
                : base(message) { }
        }
    }
}