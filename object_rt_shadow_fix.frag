#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;

out vec4 finalColor;

void main()
{
    vec4 texel = texture(texture0, fragTexCoord);

    if (texel.a > 0.0 && texel.a < 1.0)
    {
        finalColor = vec4(0.5, 0.1, 0.5, 0.4);
    }
    else
    {
        finalColor = texel;
    }
}