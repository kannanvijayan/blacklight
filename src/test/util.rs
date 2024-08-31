/**
 * Utility routines for testing.
 */

use wgpu;
use futures;

/** Obtain a new wgpu device and queue and return it. */
pub(crate) fn get_device_and_queue() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = futures::executor::block_on(
      instance.request_adapter(&wgpu::RequestAdapterOptions::default())
    ).unwrap();
    let (device, queue) = futures::executor::block_on(adapter.request_device(
      &wgpu::DeviceDescriptor::default(),
      None
    )).unwrap();
    (device, queue)
}
