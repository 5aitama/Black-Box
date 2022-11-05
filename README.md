# GraphicsRS
An exerimental project with graphics api and other stuffs.

## NOTES

* ### Buffers
    Add to the `trait RendererTrait` theses methods to expose to the user the ability to create/update/destroy gpu buffers.

    ```rust
    /// Create a buffer.
    /// 
    /// # Arguments
    /// 
    /// * `size`    - The size of the buffer.
    /// * `usage`   - The buffer usage(s).
    /// 
    fn create_buffer<T>(&mut self, size: usize, usage: BufferUsage) -> Buffer;

    /// Update the buffer data.
    /// 
    /// # Arguments
    /// 
    /// * `buffer`  - The buffer to update.
    /// * `data`    - The data to copy from.
    /// * `offset`  - The start index at where the data must be copied.
    /// 
    fn update_buffer<T>(&self, buffer: Buffer, data: Option<T>, offset: u32);

    /// Destory a buffer.
    /// 
    /// # Arguments
    /// 
    /// * `buffer` - The buffer to destory.
    /// 
    fn destroy_buffer(&mut self, buffer: Buffer);
    ```

    #### Details
    
    - The cons here is that the user **can't update** the buffer **in the logic loop only in the render loop** ! So I don't know if this way is the good way...

    - The type `T` in the buffer funtions must be compatible with `bytemuck` !