#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;

out vec4 finalColor;

// time of day
uniform float red_tint;
uniform float blue_tint;
uniform float brightness_modifier;

// stuff for world pos
uniform vec2 cameraTarget;
uniform vec2 cameraOffset;
uniform vec2 renderTargetRes;

// stuff for lighting
const int MAX_LIGHTS = 100;
uniform int lightCount;
uniform vec3 lightPosition[MAX_LIGHTS];
uniform vec3 lightColor[MAX_LIGHTS];
uniform float lightIntensity[MAX_LIGHTS];
uniform float lightRadius[MAX_LIGHTS];

void main() {
    vec4 tex = texture(texture0, fragTexCoord);

    vec4 time_of_day_tint = vec4(red_tint, 0.0, blue_tint, 0.0);
    time_of_day_tint.rgb += brightness_modifier;

    vec2 screenPosition = gl_FragCoord.xy;
    screenPosition.y = renderTargetRes.y - screenPosition.y;
    vec2 worldPosition = cameraTarget + (screenPosition - cameraOffset);

    vec3 additive_light = vec3(0.0);
    vec3 color = tex.rgb;
    vec3 ambient_lighting = vec3(1.0);

    for (int i = 0; i < lightCount; i++) {
        vec3 deltaToLight = vec3(lightPosition[i].xy - worldPosition, lightPosition[i].z);
        float distance = length(deltaToLight);
        float attenuation = 1.0 - smoothstep(0.0, lightRadius[i], distance);
        float brightness = attenuation * lightIntensity[i];
        additive_light += lightColor[i] * brightness;
    }

    vec3 lighting = ambient_lighting + additive_light;
    vec3 finalRGB = color * lighting;
    
    finalColor = vec4(finalRGB * fragColor.rgb + time_of_day_tint.rgb, tex.a);
}