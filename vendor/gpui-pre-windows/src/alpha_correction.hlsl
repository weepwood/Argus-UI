// Adapted from https://github.com/microsoft/terminal/blob/1283c0f5b99a2961673249fa77c6b986efb5086c/src/renderer/atlas/dwrite.hlsl
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

float color_brightness(float3 color) {
    // REC. 601 luminance coefficients for perceived brightness
    return dot(color, float3(0.30f, 0.59f, 0.11f));
}

float light_on_dark_contrast(float enhancedContrast, float3 color) {
    float brightness = color_brightness(color);
    float multiplier = saturate(4.0f * (0.75f - brightness));
    return enhancedContrast * multiplier;
}

// Dark ink on a light background follows Skia's mask gamma, the model Chromium
// uses on Windows: gamma 1.2, contrast 0.2, text luminance quantised to 3 bits
// and the background taken as its perceptual inverse. The DirectWrite path
// below renders dark text noticeably heavier on light surfaces.
#define MASK_GAMMA 1.2f
#define MASK_CONTRAST 0.2f
float mask_luminance(float3 ink) {
    float3 linearized = pow(saturate(ink), MASK_GAMMA);
    return pow(dot(linearized, float3(0.2126f, 0.7152f, 0.0722f)), 1.0f / MASK_GAMMA);
}
float mask_gamma(float coverage, float3 ink) {
    float level = min(floor(mask_luminance(ink) * 8.0f), 7.0f);
    float src = level / 7.0f;
    float dst = 1.0f - src;
    float src_linear = pow(src, MASK_GAMMA);
    float dst_linear = pow(dst, MASK_GAMMA);
    float blended = coverage + (1.0f - coverage) * MASK_CONTRAST * dst_linear * coverage;
    if (abs(src - dst) < 1.0f / 256.0f) {
        return blended;
    }
    float value = pow(src_linear * blended + (1.0f - blended) * dst_linear, 1.0f / MASK_GAMMA);
    return saturate((value - dst) / (src - dst));
}

float enhance_contrast(float alpha, float k) {
    return alpha * (k + 1.0f) / (alpha * k + 1.0f);
}

float3 enhance_contrast3(float3 alpha, float k) {
    return alpha * (k + 1.0f) / (alpha * k + 1.0f);
}

float apply_alpha_correction(float a, float b, float4 g) {
    float brightness_adjustment = g.x * b + g.y;
    float correction = brightness_adjustment * a + (g.z * b + g.w);
    return a + a * (1.0f - a) * correction;
}

float3 apply_alpha_correction3(float3 a, float3 b, float4 g) {
    float3 brightness_adjustment = g.x * b + g.y;
    float3 correction = brightness_adjustment * a + (g.z * b + g.w);
    return a + a * (1.0f - a) * correction;
}

float apply_contrast_and_gamma_correction(float sample, float3 color, float enhanced_contrast_factor, float4 gamma_ratios) {
    if (color_brightness(color) < 0.5f) {
        return mask_gamma(sample, color);
    }
    float enhanced_contrast = light_on_dark_contrast(enhanced_contrast_factor, color);
    float brightness = color_brightness(color);

    float contrasted = enhance_contrast(sample, enhanced_contrast);
    return apply_alpha_correction(contrasted, brightness, gamma_ratios);
}

float3 apply_contrast_and_gamma_correction3(float3 sample, float3 color, float enhanced_contrast_factor, float4 gamma_ratios) {
    float enhanced_contrast = light_on_dark_contrast(enhanced_contrast_factor, color);

    float3 contrasted = enhance_contrast3(sample, enhanced_contrast);
    return apply_alpha_correction3(contrasted, color, gamma_ratios);
}
