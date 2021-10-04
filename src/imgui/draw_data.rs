use std::slice::from_raw_parts;
use imgui_sys::{ImDrawCmd, ImDrawData, ImDrawIdx, ImDrawList, ImDrawVert};

pub(in crate::imgui) struct DrawData {
    pub(in crate::imgui) draw_data : *mut ImDrawData
}

impl DrawData {
    pub(in crate::imgui) fn raw(&self) -> &ImDrawData {
        unsafe { &*self.draw_data }
    }

    #[allow(dead_code)]
    pub(in crate::imgui) fn raw_mut(&mut self) -> &mut ImDrawData {
        unsafe { &mut *self.draw_data }
    }

    pub(in crate::imgui) fn draw_list(&self) -> DrawListIter<'_> {
        DrawListIter{
            draw_list: self.raw().CmdLists,
            count: self.raw().CmdListsCount as _,
            _marker: Default::default()
        }
    }
}

pub(in crate::imgui) struct DrawListIter<'a> {
    draw_list : *mut *mut ImDrawList,
    count : usize,
    _marker : std::marker::PhantomData<&'a u8>
}

impl<'a> Iterator for DrawListIter<'a> {
    type Item = DrawList<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let draw_list = unsafe { &**self.draw_list };
            self.count -= 1;
            Some(DrawList{
                draw_list
            })
        } else {
            Option::None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count,Some(self.count))
    }
}

pub(in crate::imgui) struct DrawList<'a> {
    draw_list: &'a ImDrawList
}

impl<'a> DrawList<'a> {
    pub(in crate::imgui) fn vertex_buffer(&self) -> &[ImDrawVert]{
        let vertex_buffer = &self.draw_list.VtxBuffer;
        unsafe {
            from_raw_parts(vertex_buffer.Data,vertex_buffer.Size as _)
        }
    }

    pub(in crate::imgui) fn index_buffer(&self) -> &[ImDrawIdx]{
        let index_buffer = &self.draw_list.IdxBuffer;
        unsafe {
            from_raw_parts(index_buffer.Data,index_buffer.Size as _)
        }
    }

    pub(in crate::imgui) fn draw_cmds(&self) -> &[ImDrawCmd] {
        let draw_cmds = &self.draw_list.CmdBuffer;
        unsafe {
            from_raw_parts(draw_cmds.Data,draw_cmds.Size as _)
        }
    }
}