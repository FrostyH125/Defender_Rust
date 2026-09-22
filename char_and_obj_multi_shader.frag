#version 330

// note from the author to whoever may be reading this:
//  i know the comments in this file are quite basic
//  this was my first time using and understanding normal maps ever, so it was a learning process


in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 finalColor;

uniform sampler2D texture0;

// time of day stuff
uniform float red_tint;
uniform float blue_tint;
uniform float brightness_modifier;

// stuff for world pos
uniform vec2 cameraTarget;
uniform vec2 cameraOffset;
uniform vec2 renderTargetRes;

// stuff for lighting
uniform sampler2D normalMap;
uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform float lightIntensity;
uniform float lightRadius;

void main()
{
    vec4 tex = texture(texture0, fragTexCoord);
    
    // is shadow
    if (fragColor.b == 0.0) {
        finalColor = vec4(0.5, 0.1, 0.5, tex.a * 0.4);
        return;
    }

    // is hovering
    if (fragColor.a == 0.0) {
        finalColor = vec4(1.0, 1.0, 1.0, tex.a);
        return;
    } 

    // is selected
    if (fragColor.g == 0) {
        finalColor = vec4(tex.r + 0.2, tex.g + 0.2, tex.b + 0.2, tex.a);
        return;
    }

    // is hovering for move
    if (fragColor.r == 0) {
        finalColor = vec4(0.9, 0.9, 0.1, tex.a);
        return;
    }

    vec4 time_of_day_tint = vec4(red_tint, 0.0, blue_tint, 0.0);
    time_of_day_tint.rgb += brightness_modifier;
    
    // find world pos of this fragment
    vec2 screenPosition = gl_FragCoord.xy;
    screenPosition.y = renderTargetRes.y - screenPosition.y;
    vec2 worldPosition = cameraTarget + (screenPosition - cameraOffset);

    // extract normal color and pixel color of fragment
    vec3 color = tex.rgb;
    vec3 normal = texture(normalMap, fragTexCoord).rgb;
    normal = normal * 2.0 - 1.0;

    
    // determine the direction of the light
    vec3 lightDirection = normalize(vec3(lightPosition.xy - worldPosition, lightPosition.z));

    // determine brightness using the normal color and the direction of the lighting
    float brightness = dot(normal, lightDirection);
    float distance = length(lightPosition.xy - worldPosition);
    float attenuation = 1.0 - smoothstep(0.0, lightRadius, distance);
    brightness = max(brightness, 0.0);
    brightness *= attenuation;

    // determine the lighting based on light color and brightness
    vec3 lighting = lightColor * brightness;

    // multiply the original fragments pixel color with the lighting from the light
    vec3 finalRGB = color * lighting;
    
    finalColor = vec4(finalRGB * fragColor.rgb + time_of_day_tint.rgb, tex.a);
    //finalColor = vec4(normal, tex.a);
}
