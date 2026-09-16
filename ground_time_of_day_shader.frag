#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

out vec4 finalColor;

uniform float red_tint;
uniform float blue_tint;
uniform float brightness_modifier;

void main() {
    vec4 tex = texture(texture0, fragTexCoord);

    vec4 time_of_day_tint = vec4(red_tint, 0.0, blue_tint, 0.0);
    time_of_day_tint.rgb += brightness_modifier;

    finalColor = tex * fragColor * colDiffuse + time_of_day_tint;
}