//! Tests for metadata context functionality

use super::context::*;
use crate::metadata::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_context_new() {
        let context = MetadataContext::new();
        assert!(context.parent.is_none());
        assert!(context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_metadata_context_with_config() {
        let config = MetadataConfig::default();
        let context = MetadataContext::with_config(config.clone());
        assert_eq!(context.config.as_ref(), &config);
        assert!(context.parent.is_none());
    }

    #[test]
    fn test_metadata_context_with_parent() {
        let parent = Arc::new(MetadataContext::new());
        let child = MetadataContext::with_parent(parent.clone());
        assert!(child.parent.is_some());
        assert_eq!(child.config.as_ref(), parent.config.as_ref());
    }

    #[test]
    fn test_push_and_pop_metadata() {
        let context = MetadataContext::new();
        let metadata = Metadata::builder()
            .title("Test Title")
            .description("Test Description")
            .build();

        // Initially empty
        assert!(context.metadata_stack.read().is_empty());

        // Push metadata
        context.push_metadata(metadata.clone());
        assert_eq!(context.metadata_stack.read().len(), 1);

        // Pop metadata
        let popped = context.pop_metadata();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().title, metadata.title);
        assert!(context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_get_merged_metadata_empty() {
        let context = MetadataContext::new();
        let merged = context.get_merged_metadata();
        assert_eq!(merged, Metadata::default());
    }

    #[test]
    fn test_get_merged_metadata_with_stack() {
        let context = MetadataContext::new();
        let metadata1 = Metadata::builder()
            .title("Title 1")
            .description("Description 1")
            .build();
        let metadata2 = Metadata::builder()
            .title("Title 2")
            .keywords(vec!["keyword1".to_string()])
            .build();

        context.push_metadata(metadata1);
        context.push_metadata(metadata2);

        let merged = context.get_merged_metadata();
        assert_eq!(merged.title, Some("Title 2".to_string())); // Last title wins
        assert_eq!(merged.description, Some("Description 1".to_string()));
        assert_eq!(merged.keywords, Some(vec!["keyword1".to_string()]));
    }

    #[test]
    fn test_get_merged_metadata_with_parent() {
        let parent = Arc::new(MetadataContext::new());
        let parent_metadata = Metadata::builder()
            .title("Parent Title")
            .description("Parent Description")
            .build();
        parent.push_metadata(parent_metadata);

        let child = MetadataContext::with_parent(parent);
        let child_metadata = Metadata::builder()
            .title("Child Title")
            .keywords(vec!["child_keyword".to_string()])
            .build();
        child.push_metadata(child_metadata);

        let merged = child.get_merged_metadata();
        assert_eq!(merged.title, Some("Child Title".to_string())); // Child overrides parent
        assert_eq!(merged.description, Some("Parent Description".to_string())); // Inherited from parent
        assert_eq!(merged.keywords, Some(vec!["child_keyword".to_string()])); // From child
    }

    #[test]
    fn test_update_config() {
        let mut context = MetadataContext::new();
        let new_config = MetadataConfig::default();

        context.update_config(new_config.clone());
        assert_eq!(context.config.as_ref(), &new_config);
    }

    #[test]
    fn test_metadata_context_default() {
        let context = MetadataContext::default();
        assert!(context.parent.is_none());
        assert!(context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_provide_metadata_context() {
        let context = provide_metadata_context();
        assert!(context.parent.is_none());
        assert!(context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_metadata_provider_new() {
        let provider = MetadataProvider::new();
        assert!(provider.context.parent.is_none());
        assert!(provider.context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_metadata_provider_with_config() {
        let config = MetadataConfig::default();
        let provider = MetadataProvider::with_config(config.clone());
        assert_eq!(provider.context.config.as_ref(), &config);
    }

    #[test]
    fn test_metadata_provider_default() {
        let provider = MetadataProvider::default();
        assert!(provider.context.parent.is_none());
        assert!(provider.context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_metadata_provider_get_context() {
        let provider = MetadataProvider::new();
        let context = provider.get_context();
        assert!(context.parent.is_none());
        assert!(context.metadata_stack.read().is_empty());
    }

    #[test]
    fn test_metadata_provider_context_operations() {
        let provider = MetadataProvider::new();
        let context = provider.get_context();

        let metadata = Metadata::builder()
            .title("Provider Test")
            .build();

        context.push_metadata(metadata);
        let merged = context.get_merged_metadata();
        assert_eq!(merged.title, Some("Provider Test".to_string()));
    }

    #[test]
    fn test_nested_context_inheritance() {
        // Create grandparent
        let grandparent = Arc::new(MetadataContext::new());
        let grandparent_metadata = Metadata::builder()
            .title("Grandparent Title")
            .description("Grandparent Description")
            .build();
        grandparent.push_metadata(grandparent_metadata);

        // Create parent
        let parent = Arc::new(MetadataContext::with_parent(grandparent));
        let parent_metadata = Metadata::builder()
            .title("Parent Title")
            .keywords(vec!["parent_keyword".to_string()])
            .build();
        parent.push_metadata(parent_metadata);

        // Create child
        let child = MetadataContext::with_parent(parent);
        let child_metadata = Metadata::builder()
            .title("Child Title")
            .build();
        child.push_metadata(child_metadata);

        let merged = child.get_merged_metadata();
        assert_eq!(merged.title, Some("Child Title".to_string())); // Child wins
        assert_eq!(merged.description, Some("Grandparent Description".to_string())); // From grandparent
        assert_eq!(merged.keywords, Some(vec!["parent_keyword".to_string()])); // From parent
    }

    #[test]
    fn test_multiple_metadata_in_stack() {
        let context = MetadataContext::new();

        // Push multiple metadata entries
        context.push_metadata(Metadata::builder().title("First").build());
        context.push_metadata(Metadata::builder().description("Second").build());
        context.push_metadata(Metadata::builder().keywords(vec!["third".to_string()]).build());

        let merged = context.get_merged_metadata();
        assert_eq!(merged.title, Some("First".to_string()));
        assert_eq!(merged.description, Some("Second".to_string()));
        assert_eq!(merged.keywords, Some(vec!["third".to_string()]));
    }
}
