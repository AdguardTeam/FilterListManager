// ReSharper disable CheckNamespace

using AdGuard.FilterListManagerProtobuf.Api;

// ReSharper disable ClassNeverInstantiated.Global


namespace FilterListManager
{
    // Partial classed below have the only purpose to allow inheritance from IAGOuterError
    // in order to gentle handle in generic manner in CallRust method
    
    // ReSharper disable once InconsistentNaming
    public partial class AGOuterError : IAGOuterError
    {
        public AGOuterError Error { get; set; }
    }

    public partial class Configuration : IAGOuterError
    {
        public AGOuterError Error { get; set; }
    }

    public partial class InstallCustomFilterListResponse : IAGOuterError
    {
    }
    
    public partial class EnableFilterListsResponse : IAGOuterError
    {
    }
    
    public partial class InstallFilterListsResponse : IAGOuterError
    {
    }
    
    public partial class DeleteCustomFilterListsResponse : IAGOuterError
    {
    }
    
    public partial class GetStoredFiltersMetadataResponse : IAGOuterError
    {
    }
    
    public partial class GetStoredFilterMetadataByIdResponse : IAGOuterError
    {
    }
    
    public partial class GetFullFilterListByIdResponse : IAGOuterError
    {
    }
    
    public partial class UpdateFiltersResponse : IAGOuterError
    {
    }
    
    public partial class ForceUpdateFiltersByIdsResponse : IAGOuterError
    {
    }
    
    public partial class FetchFilterListMetadataResponse : IAGOuterError
    {
    }

    public partial class FetchFilterListMetadataWithBodyResponse : IAGOuterError
    {
    }

    public partial class GetAllTagsResponse : IAGOuterError
    {
    }

    public partial class GetAllGroupsResponse : IAGOuterError
    {
    }

    public partial class ChangeLocaleResponse : IAGOuterError 
    {
    }

    public partial class UpdateCustomFilterMetadataResponse : IAGOuterError 
    {
    }

    public partial class GetDatabasePathResponse : IAGOuterError 
    {
    }

    public partial class GetDatabaseVersionResponse : IAGOuterError 
    {
    }

    public partial class InstallCustomFilterFromStringResponse : IAGOuterError 
    {
    }

    public partial class GetActiveRulesResponse : IAGOuterError 
    {
    }

    public partial class EmptyResponse : IAGOuterError 
    {
    }
    
    public partial class GetFilterRulesAsStringsResponse : IAGOuterError 
    {
    }
    
    public partial class GetDisabledRulesResponse : IAGOuterError 
    {
    }

    public partial class GetRulesCountResponse : IAGOuterError
    {
    }
}

