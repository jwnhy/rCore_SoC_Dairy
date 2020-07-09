use lazy_static::lazy_static;
lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: UnsafeWrapper<Processor> = Default::default();
}

