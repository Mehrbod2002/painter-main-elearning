// Shared Input and Output Structures
struct VertexInput {
    @location(0) position: vec2<f32>, // 2D position
    @location(1) color: vec4<f32>,    // RGBA color
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Transformed position
    @location(0) color: vec4<f32>,          // Passed color
};

// ==================== TRIANGLE SHADER ====================

// Vertex Shader for Triangle
@vertex
fn triangle_vs(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Pass-through vertex position and color
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;

    return output;
}

// Vertex Shader
@vertex
fn rectangle_vs(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Pass the position to clip space
    output.position = vec4<f32>(input.position, 0.0, 1.0);

    // Pass the color to the fragment shader
    output.color = input.color;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Output the interpolated color
    return input.color;
}