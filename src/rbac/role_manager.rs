use crate::Result;

pub type MatchingFn = fn(&str, &str) -> bool;

pub trait RoleManager: Send + Sync {
    fn clear(&mut self);
    fn add_link(&mut self, name1: &str, name2: &str, domain: Option<&str>);
    #[deprecated]
    fn matching_fn(
        &mut self,
        role_matching_fn: Option<MatchingFn>,
        domain_matching_fn: Option<MatchingFn>,
    );

    fn set_role_matching_fn(&mut self, role_matching_fn: MatchingFn);

    fn set_domain_matching_fn(&mut self, domain_matching_fn: MatchingFn);

    fn delete_link(
        &mut self,
        name1: &str,
        name2: &str,
        domain: Option<&str>,
    ) -> Result<()>;
    fn has_link(
        &mut self,
        name1: &str,
        name2: &str,
        domain: Option<&str>,
    ) -> bool;
    fn get_roles(&self, name: &str, domain: Option<&str>) -> Vec<String>;
    fn get_users(&self, name: &str, domain: Option<&str>) -> Vec<String>;
}
