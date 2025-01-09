// shader.wgsl

struct VertexInput {
    @location(0) position: vec2<f32>, // 2D position
    @location(1) color: vec4<f32>,    // RGBA color
};

// Vertex Output Structure
struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Transformed position
    @location(0) color: vec4<f32>,          // Passed color
};

// Vertex Shader Entry Point
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Transform 2D position to 4D clip space
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    
    // Pass color to fragment shader
    output.color = input.color;
    
    return output;
}

// Fragment Shader Entry Point
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Output the interpolated color
    return input.color;
}
