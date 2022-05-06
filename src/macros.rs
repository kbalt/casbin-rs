#[macro_export]
macro_rules! get_or_err {
    ($this:ident, $key:expr, $err:expr, $msg:expr) => {{
        $this
            .get_model()
            .get_model()
            .get($key)
            .ok_or_else(|| {
                $crate::error::Error::from($err(format!(
                    "Missing {} definition in conf file",
                    $msg
                )))
            })?
            .get($key)
            .ok_or_else(|| {
                $crate::error::Error::from($err(format!(
                    "Missing {} section in conf file",
                    $msg
                )))
            })?
    }};
}

#[macro_export]
macro_rules! push_index_if_explain {
    ($this:ident) => {{
        #[cfg(feature = "explain")]
        if $this.cap > 1 {
            $this.expl.push($this.idx);
        }
    }};
}
