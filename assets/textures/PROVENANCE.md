# Weathered rock

Файл: `weathered-rock.png`.

Создан встроенным инструментом `image_gen` через навык imagegen для материала скал «Кромки бури». Исходный результат скопирован в проект без редактирования. Mip-уровни строятся кодом загрузки текстуры; трипланарное наложение, смешивание образцов, рельеф и мох реализованы в WGSL.

Полный промпт:

> Use case: stylized-concept. Asset type: one square seamless tileable diffuse albedo texture for natural stone cliffs in a polished real-time 3D exploration game. Generate a 1024 by 1024 square full-frame surface swatch of weathered cool gray stone, a natural mix of basalt and pale limestone. Irregular medium-scale broken rock faces, subtle sediment layers and hairline cracks, fine mineral grain, soft worn edges, occasional muted gray-green mineral stains. Rich believable small surface detail, moderate contrast, restrained gray and cool taupe colors. Orthographic straight-on view of a flat material surface, perfectly uniform soft neutral illumination, no perspective, no directional lighting, no baked shadows, no ambient occlusion borders, no specular highlights. The four edges should tile seamlessly. No plants, no grass, no moss patches, no water, no objects, no text, no labels, no frames, no watermark. Avoid vertical stripes, long straight seams, regular brick patterns, oversized dark cracks, isolated recognizable motifs, shiny plastic, or a sculpted boulder silhouette. This is a texture map, not concept art or a landscape.
