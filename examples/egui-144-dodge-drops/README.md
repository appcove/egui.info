
# egui-144-circle-chaser

This is an example of keypressing to effect a circle, and using Pos2.distance

## Source
- [src/main.rs]({{ site.codeurl }}/examples/egui-144-circle-chaser/src/main.rs)
- [Project Directory]({{ site.codeurl }}/examples/egui-144-circle-chaser)


## Screenshots
![screenshot](screen1.png)
![screenshot](screen2.png)


## Description

When creating the ExampleApp struct, we add two values to hold the position of the circles, and values to hold the size, speed, and color. 

```rust
struct ExampleApp {
    cx: f32,
    cy: f32,
    cs: f32,
    cc: Color32,
    sx: f32,
    sy: f32,
    tx: f32,
    ty: f32,
    ts: f32,
    dd: f32,
}
```

These values need initialized in the `Default` trait implementation. Rust will call the `default()` function and expect to get a fully initialized structure back as the return value.

```rust
impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            cx: 250.0,
            cy: 250.0,
            cs: 20.0,
            cc: Color32::BLUE,
            sx: 0.0,
            sy: 0.0,
            tx: 500.0,
            ty: 500.0,
            ts: 50.0,
            dd: 0.0,
        }
    }
}
```

We use `Pos2.distance` to get the distance from the center of our circle to the center of our target circle.

If the distance is less than the sum of both circles radius (`self.cs` & `self.ts`), then we move our target circle to a random position on the screen
```rust
self.dd = self.cs + self.ts;

if (egui::Pos2{x:self.cx,y:self.cy}).distance(egui::Pos2{x:self.tx,y:self.ty}) < self.dd {
    self.tx = rand::thread_rng().gen_range(0.0..1000.0);
    self.ty = rand::thread_rng().gen_range(0.0..1000.0);    
}
```



