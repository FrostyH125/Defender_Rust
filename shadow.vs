in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec4 vertexColor;

out vec2 fragTexCoord;
out vec4 fragColor;

uniform float shear;
uniform float scale_y;
uniform mat4 mvp;

void main() {
    bool is_shadow = vertexColor.a < 0.5;
    vec3 pos = vertexPosition;

    if (is_shadow) {
        // shear the sprite by moving the top vertices only
        pos.x += (1.0 - vertexTexCoord.y) * shear;
        pos.y += (1.0 - vertexTexCoord.y) * scale_y;
    }

    fragTexCoord = vertexTexCoord;
    fragColor = vertexColor;
    gl_Position = mvp * vec4(pos, 1.0);
}