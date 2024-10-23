pub struct TagUsecase {}

impl TagUsecase {
    pub fn new() -> anyhow::Result<Self> {
        Ok(TagUsecase {})
    }
    pub fn list_tag(&self) -> anyhow::Result<()> {
        Ok(())
    }
    pub fn create_tag(&self) -> anyhow::Result<()> {
        Ok(())
    }
    pub fn modify_tag(&self) -> anyhow::Result<()> {
        Ok(())
    }
    pub fn remove_tag(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
