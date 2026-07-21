#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 finalColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

uniform float red_tint;
uniform float blue_tint;
uniform float brightness_modifier;

void main()
{
    vec4 tex = texture(texture0, fragTexCoord);

    // is hovering
    if (fragColor.a == 0.0) {
        finalColor = vec4(1.0, 1.0, 1.0, tex.a);
        return;
    } 

    // is shadow
    if (fragColor.b == 0.0) {
        finalColor = vec4(0.6, 0.2, 0.6, tex.a * 0.5);
        return;
    }

    vec4 time_of_day_tint = vec4(red_tint, 0.0, blue_tint, 0.0);
    time_of_day_tint.rgb += brightness_modifier;
    
    finalColor = tex * fragColor * colDiffuse + time_of_day_tint;

}
