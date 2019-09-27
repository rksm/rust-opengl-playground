#version 330 core
out vec4 FragColor;

in VS_OUTPUT {
  vec3 Color;
} IN;

void main() {
    FragColor = vec4(IN.Color, 1.0f);
    // FragColor = vec4(1.0,0.0,0.0, 1.0f);
}
