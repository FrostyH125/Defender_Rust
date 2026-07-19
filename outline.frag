#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 finalColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

void main()
{
    vec4 tex = texture(texture0, fragTexCoord);

    if (fragColor.a == 0.0) {
        finalColor = vec4(1.0, 1.0, 1.0, tex.a);
        return;
    } 

    if (fragColor.b == 0.0) {
        finalColor = vec4(0.6, 0.2, 0.6, tex.a * 0.5);
        return;
    }
    
    finalColor = tex * fragColor * colDiffuse;

}
