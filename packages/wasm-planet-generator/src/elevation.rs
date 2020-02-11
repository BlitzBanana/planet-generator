
extern crate noise;

use crate::map::Point;
use noise::*;
use noise::utils::*;

#[allow(non_snake_case)]
pub fn elevate(seed: u64, points: &Vec<Point>, width: f64, height: f64) -> Vec<f64> {
  // Planet seed. Change this to generate a different planet.
  let CURRENT_SEED: u32 = seed as u32;

  /// Frequency of the planet's continents. Higher frequency produces
  /// smaller, more numerous continents. This value is measured in radians.
  const CONTINENT_FREQUENCY: f64 = 0.4;

  /// Lacunarity of the planet's continents. Changing this value produces
  /// slightly different continents. For the best results, this value should
  /// be random, but close to 2.0.
  const CONTINENT_LACUNARITY: f64 = 2.208984375;

  /// Lacunarity of the planet's mountains. Changing the value produces
  /// slightly different mountains. For the best results, this value should
  /// be random, but close to 2.0.
  const MOUNTAIN_LACUNARITY: f64 = 2.142578125;

  /// Lacunarity of the planet's hills. Changing this value produces
  /// slightly different hills. For the best results, this value should be
  /// random, but close to 2.0.
  const HILLS_LACUNARITY: f64 = 2.162109375;

  /// Lacunarity of the planet's plains. Changing this value produces
  /// slightly different plains. For the best results, this value should be
  /// random, but close to 2.0.
  const PLAINS_LACUNARITY: f64 = 2.314453125;

  /// Lacunarity of the planet's badlands. Changing this value produces
  /// slightly different badlands. For the best results, this value should
  /// be random, but close to 2.0.
  const BADLANDS_LACUNARITY: f64 = 2.212890625;

  /// Specifies the "twistiness" of the mountains.
  const MOUNTAINS_TWIST: f64 = 1.0;

  /// Specifies the "twistiness" of the hills.
  const HILLS_TWIST: f64 = 1.0;

  /// Specifies the "twistiness" of the badlands.
  const BADLANDS_TWIST: f64 = 1.0;

  /// Specifies the planet's sea level. This value must be between -1.0
  /// (minimum planet elevation) and +1.0 (maximum planet elevation).
  const SEA_LEVEL: f64 = 0.0;

  /// Specifies the level on the planet in which continental shelves appear.
  /// This value must be between -1.0 (minimum planet elevation) and +1.0
  /// (maximum planet elevation), and must be less than `SEA_LEVEL`.
  const SHELF_LEVEL: f64 = -0.375;

  /// Determines the amount of mountainous terrain that appears on the
  /// planet. Values range from 0.0 (no mountains) to 1.0 (all terrain is
  /// covered in mountains). Mountains terrain will overlap hilly terrain.
  /// Because the badlands terrain may overlap parts of the mountainous
  /// terrain, setting `MOUNTAINS_AMOUNT` to 1.0 may not completely cover the
  /// terrain in mountains.
  const MOUNTAINS_AMOUNT: f64 = 0.5;

  /// Determines the amount of hilly terrain that appears on the planet.
  /// Values range from 0.0 (no hills) to 1.0 (all terrain is covered in
  /// hills). This value must be less than `MOUNTAINS_AMOUNT`. Because the
  /// mountains terrain will overlap parts of the hilly terrain, and the
  /// badlands terrain may overlap parts of the hilly terrain, setting
  /// `HILLS_AMOUNT` to 1.0 may not completely cover the terrain in hills.
  const HILLS_AMOUNT: f64 = (1.0 + MOUNTAINS_AMOUNT) / 2.0;

  /// Determines the amount of badlands terrain that covers the planet.
  /// Values range from 0.0 (no badlands) to 1.0 (all terrain is covered in
  /// badlands). Badlands terrain will overlap any other type of terrain.
  const BADLANDS_AMOUNT: f64 = 0.3125;

  /// Offset to apply to the terrain type definition. Low values (< 1.0)
  /// cause the rough areas to appear only at high elevations. High values
  /// (> 2.0) cause the rough areas to appear at any elevation. The
  /// percentage of rough areas on the planet are independent of this value.
  const TERRAIN_OFFSET: f64 = 1.0;

  /// Specifies the amount of "glaciation" on the mountains. This value
  /// should be close to 1.0 and greater than 1.0.
  const MOUNTAIN_GLACIATION: f64 = 1.375;

  /// Scaling to apply to the base continent elevations, in planetary
  /// elevation units.
  const CONTINENT_HEIGHT_SCALE: f64 = (1.0 - SEA_LEVEL) / 4.0;

  /// Maximum depth of the rivers, in planetary elevation units.
  const RIVER_DEPTH: f64 = 0.0234375;

  // ////////////////////////////////////////////////////////////////////////
  // Function group: continent definition
  // ////////////////////////////////////////////////////////////////////////

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: base continent definition (7 noise functions)
  //
  // This subgroup roughly defines the positions and base elevations of the
  // planet's continents.
  //
  // The "base elevation" is the elevation of the terrain before any terrain
  // features (mountains, hills, etc.) are placed on that terrain.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Continent module]: This FBM module generates the continents. This
  // noise function has a high number of octaves so that detail is visible at
  // high zoom levels.
  let baseContinentDef_fb0 = Fbm::new()
      .set_seed(CURRENT_SEED)
      .set_frequency(CONTINENT_FREQUENCY)
      .set_persistence(0.5)
      .set_lacunarity(CONTINENT_LACUNARITY)
      .set_octaves(14);

  // 2: [Continent-with-ranges module]: Next, a curve module modifies the
  // output value from the continent module so that very high values appear
  // near sea level. This defines the positions of the mountain ranges.
  let baseContinentDef_cu = Curve::new(&baseContinentDef_fb0)
      .add_control_point(-2.0000 + SEA_LEVEL, -1.625 + SEA_LEVEL)
      .add_control_point(-1.0000 + SEA_LEVEL, -1.375 + SEA_LEVEL)
      .add_control_point(0.0000 + SEA_LEVEL, -0.375 + SEA_LEVEL)
      .add_control_point(0.0625 + SEA_LEVEL, 0.125 + SEA_LEVEL)
      .add_control_point(0.1250 + SEA_LEVEL, 0.250 + SEA_LEVEL)
      .add_control_point(0.2500 + SEA_LEVEL, 1.000 + SEA_LEVEL)
      .add_control_point(0.5000 + SEA_LEVEL, 0.250 + SEA_LEVEL)
      .add_control_point(0.7500 + SEA_LEVEL, 0.250 + SEA_LEVEL)
      .add_control_point(1.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL)
      .add_control_point(2.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL);


  // 3: [Carver module]: This higher-frequency BasicMulti module will be
  // used by subsequent noise functions to carve out chunks from the
  // mountain ranges within the continent-with-ranges module so that the
  // mountain ranges will not be completely impassible.
  let baseContinentDef_fb1 = Fbm::new()
      .set_seed(CURRENT_SEED + 1)
      .set_frequency(CONTINENT_FREQUENCY * 4.34375)
      .set_persistence(0.5)
      .set_lacunarity(CONTINENT_LACUNARITY)
      .set_octaves(11);


  // 4: [Scaled-carver module]: This scale/bias module scales the output
  // value from the carver module such that it is usually near 1.0. This
  // is required for step 5.
  let baseContinentDef_sb = ScaleBias::new(&baseContinentDef_fb1)
      .set_scale(0.375)
      .set_bias(0.625);


  // 5: [Carved-continent module]: This minimum-value module carves out
  // chunks from the continent-with-ranges module. it does this by ensuring
  // that only the minimum of the output values from the scaled-carver
  // module and the continent-with-ranges module contributes to the output
  // value of this subgroup. Most of the time, the minimum value module will
  // select the output value from the continent-with-ranges module since the
  // output value from the scaled-carver is usually near 1.0. Occasionally,
  // the output from the scaled-carver module will be less than the output
  // value from the continent-with-ranges module, so in this case, the output
  // value from the scaled-carver module is selected.
  let baseContinentDef_mi = Min::new(&baseContinentDef_sb, &baseContinentDef_cu);


  // 6: [Clamped-continent module]: Finally, a clamp module modifies the
  // carved continent module to ensure that the output value of this subgroup
  // is between -1.0 and 1.0.
  let baseContinentDef_cl = Clamp::new(&baseContinentDef_mi).set_bounds(-1.0, 1.0);

  // 7: [Base-continent-definition subgroup]: Caches the output value from
  // the clamped-continent module.
  let baseContinentDef = Cache::new(baseContinentDef_cl);


  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: continent definition (5 noise functions)
  //
  // This subgroup warps the output value from the base-continent-definition
  // subgroup, producing more realistic terrain.
  //
  // Warping the base continent definition produces lumpier terrain with
  // cliffs and rifts.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Coarse-turbulence module]: This turbulence module warps the output
  // value from the base-continent-definition subgroup, adding some coarse
  // detail to it.
  let continentDef_tu0 = Turbulence::new(&baseContinentDef)
      .set_seed(CURRENT_SEED + 10)
      .set_frequency(CONTINENT_FREQUENCY * 15.25)
      .set_power(CONTINENT_FREQUENCY / 113.75)
      .set_roughness(13);


  // 2: [Intermediate-turbulence module]: This turbulence module warps the
  // output value from the coarse-turbulence module. This turbulence has a
  // higher frequency, but lower power, than the coarse-turbulence module,
  // adding some intermediate detail to it.
  let continentDef_tu1 = Turbulence::new(continentDef_tu0)
      .set_seed(CURRENT_SEED + 11)
      .set_frequency(CONTINENT_FREQUENCY * 47.25)
      .set_power(CONTINENT_FREQUENCY / 433.75)
      .set_roughness(12);


  // 3: [Warped-base-continent-definition module]: This turbulence module
  // warps the output value from the intermediate-turbulence module. This
  // turbulence has a higher frequency, but lower power, than the
  // intermediate-turbulence module, adding some fine detail to it.
  let continentDef_tu2 = Turbulence::new(continentDef_tu1)
      .set_seed(CURRENT_SEED + 12)
      .set_frequency(CONTINENT_FREQUENCY * 95.25)
      .set_power(CONTINENT_FREQUENCY / 1019.75)
      .set_roughness(11);


  // 4: [Select-turbulence module]: At this stage, the turbulence is applied
  // to the entire base-continent-definition subgroup, producing some very
  // rugged, unrealistic coastlines.  This selector module selects the
  // output values from the (unwarped) base-continent-definition subgroup
  // and the warped-base-continent-definition module, based on the output
  // value from the (unwarped) base-continent-definition subgroup.  The
  // selection boundary is near sea level and has a relatively smooth
  // transition.  In effect, only the higher areas of the base-continent-
  // definition subgroup become warped; the underwater and coastal areas
  // remain unaffected.
  let continentDef_se = Select::new(&baseContinentDef, &continentDef_tu2, &baseContinentDef)
      .set_bounds(SEA_LEVEL - 0.0375, SEA_LEVEL + 1000.0375)
      .set_falloff(0.0625);


  // 5: [Continent-definition group]: Caches the output value from the
  // clamped-continent module. This is the output value for the entire
  // continent-definition group.
  let continentDef = Cache::new(continentDef_se);

  // ////////////////////////////////////////////////////////////////////////
  // Function group: terrain type definition
  // ////////////////////////////////////////////////////////////////////////

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: terrain type definition (3 noise functions)
  //
  // This subgroup defines the positions of the terrain types on the planet.
  //
  // Terrain types include, in order of increasing roughness, plains, hills,
  // and mountains.
  //
  // This subgroup's output value is based on the output value from the
  // continent-definition group. Rougher terrain mainly appears at higher
  // elevations.
  //
  // -1.0 represents the smoothest terrain types (plains and underwater) and
  // +1.0 represents the roughest terrain types (mountains).
  //

  // 1: [Warped-continent module]: This turbulence module slightly warps the
  // output value from the continent-definition group. This prevents the
  // rougher terrain from appearing exclusively at higher elevations. Rough
  // areas may now appear in the the ocean, creating rocky islands and
  // fjords.
  let terrainTypeDef_tu = Turbulence::new(&continentDef)
      .set_seed(CURRENT_SEED + 20)
      .set_frequency(CONTINENT_FREQUENCY * 18.125)
      .set_power(CONTINENT_FREQUENCY / 20.59375 * TERRAIN_OFFSET)
      .set_roughness(3);

  // 2: [Roughness-probability-shift module]: This terracing module sharpens
  // the edges of the warped-continent module near sea level and lowers the
  // slope towards the higher-elevation areas. This shrinks the areas in
  // which the rough terrain appears, increasing the "rarity" of rough
  // terrain.
  let terrainTypeDef_te = Terrace::new(&terrainTypeDef_tu)
      .add_control_point(-1.00)
      .add_control_point(SHELF_LEVEL + SEA_LEVEL / 2.0)
      .add_control_point(1.00);

  // 3: [Terrain-type-definition group]: Caches the output value from the
  // roughness-probability-shift module. This is the output value for the
  // entire terrain-type-definition group.
  let terrainTypeDef = Cache::new(terrainTypeDef_te);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: mountainous terrain
  // /////////////////////////////////////////////////////////////////////////

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: mountain base definition (9 noise functions)
  //
  // This subgroup generates the base-mountain elevations. Other subgroups
  // will add the ridges and low areas to the base elevations.
  //
  // -1.0 represents low mountainous terrain and +1.0 represents high
  // mountainous terrain.
  //

  // 1: [Mountain-ridge module]: This ridged-multifractal-noise function
  // generates the mountain ridges.
  let mountainBaseDef_rm0 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 30)
      .set_frequency(1723.0)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(4);

  // 2: [Scaled-mountain-ridge module]: Next, a scale/bias module scales the
  // output value from the mountain-ridge module so that its ridges are not
  // too high. The reason for this is that another subgroup adds actual
  // mountainous terrain to these ridges.
  let mountainBaseDef_sb0 = ScaleBias::new(&mountainBaseDef_rm0)
      .set_scale(0.5)
      .set_bias(0.375);

  // 3: [River-valley module]: This ridged-multifractal-noise function
  // generates the river valleys.  It has a much lower frequency than the
  // mountain-ridge module so that more mountain ridges will appear outside
  // of the valleys. Note that this noise function generates ridged-multifractal
  // noise using only one octave; this information will be important in the
  // next step.
  let mountainBaseDef_rm1 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 31)
      .set_frequency(367.0)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(1);

  // 4: [Scaled-river-valley module]: Next, a scale/bias module applies a
  // scaling factor of -2.0 to the output value from the river-valley module.
  // This stretches the possible elevation values because one-octave ridged-
  // multifractal noise has a lower range of output values than multiple-
  // octave ridged-multifractal noise. The negative scaling factor inverts
  // the range of the output value, turning the ridges from the river-valley
  // module into valleys.
  let mountainBaseDef_sb1 = ScaleBias::new(&mountainBaseDef_rm1)
      .set_scale(-2.0)
      .set_bias(-0.5);

  // 5: [Low-flat module]: This low constant value is used by step 6.
  let mountainBaseDef_co = Constant::new(-1.0);

  // 6: [Mountains-and-valleys module]: This blender module merges the scaled-
  // mountain-ridge module and the scaled-river-valley module together. It
  // causes the low-lying areas of the terrain to become smooth, and causes
  // the high-lying areas of the terrain to contain ridges. To do this, it
  // uses the scaled-river-valley module as the control module, causing the
  // low-flat module to appear in the lower areas and causing the scaled-
  // mountain-ridge module to appear in the higher areas.
  let mountainBaseDef_bl = Blend::new(
      &mountainBaseDef_co,
      &mountainBaseDef_sb0,
      &mountainBaseDef_sb1,
  );

  // 7: [Coarse-turbulence module]: This turbulence module warps the output
  // value from the mountain-and-valleys module, adding some coarse detail to
  // it.
  let mountainBaseDef_tu0 = Turbulence::new(mountainBaseDef_bl)
      .set_seed(CURRENT_SEED + 32)
      .set_frequency(1337.0)
      .set_power(1.0 / 6730.0 * MOUNTAINS_TWIST)
      .set_roughness(4);

  // 8: [Warped-mountains-and-valleys module]: This turbulence module warps
  // the output value from the coarse-turbulence module. This turbulence has
  // a higher frequency, but lower power, than the coarse-turbulence module,
  // adding some fine detail to it.
  let mountainBaseDef_tu1 = Turbulence::new(mountainBaseDef_tu0)
      .set_seed(CURRENT_SEED + 33)
      .set_frequency(21221.0)
      .set_power(1.0 / 120157.0 * MOUNTAINS_TWIST)
      .set_roughness(6);

  // 9: [Mountain-base-definition subgroup]: Caches the output value from the
  // warped-mountains-and-valleys module.
  let mountainBaseDef = Cache::new(mountainBaseDef_tu1);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: high mountainous terrain (5 noise functions)
  //
  // This subgroup generates the mountainous terrain that appears at high
  // elevations within the mountain ridges.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Mountain-basis-0 module]: This ridged-multifractal-noise function,
  // along with the mountain-basis-1 module, generates the individual
  // mountains.
  let mountainousHigh_rm0 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 40)
      .set_frequency(2371.0)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(3);

  // 2: [Mountain-basis-1 module]: This ridged-multifractal-noise function,
  // along with the mountain-basis-0 module, generates the individual
  // mountains.
  let mountainousHigh_rm1 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 41)
      .set_frequency(2341.0)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(3);

  // 3: [High-mountains module]: Next, a maximum-value module causes more
  // mountains to appear at the expense of valleys. It does this by ensuring
  // that only the maximum of the output values from the two ridged-
  // multifractal-noise functions contribute to the output value of this
  // subgroup.
  let mountainousHigh_ma = Max::new(&mountainousHigh_rm0, &mountainousHigh_rm1);

  // 4: [Warped-high-mountains module]: This turbulence module warps the
  // output value from the high-mountains module, adding some detail to it.
  let mountainousHigh_tu = Turbulence::new(mountainousHigh_ma)
      .set_seed(CURRENT_SEED + 42)
      .set_frequency(31511.0)
      .set_power(1.0 / 180371.0 * MOUNTAINS_TWIST)
      .set_roughness(4);

  // 5: [High-mountainous-terrain subgroup]: Caches the output value from the
  // warped-high-mountains module.
  let mountainousHigh = Cache::new(mountainousHigh_tu);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: low mountainous terrain (4 noise functions)
  //
  // This subgroup generates the mountainous terrain that appears at low
  // elevations within the river valleys.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Lowland-basis-0 module]: This ridged-multifractal-noise function,
  // along with the lowland-basis-1 module, produces the low mountainous
  // terrain.
  let mountainousLow_rm0 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 50)
      .set_frequency(1381.0)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(8);

  // 1: [Lowland-basis-1 module]: This ridged-multifractal-noise function,
  // along with the lowland-basis-0 module, produces the low mountainous
  // terrain.
  let mountainousLow_rm1 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 51)
      .set_frequency(1427.0)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(8);

  // 3: [Low-mountainous-terrain module]: This multiplication module combines
  // the output values from the two ridged-multifractal-noise functions. This
  // causes the following to appear in the resulting terrain:
  // - Cracks appear when two negative output values are multiplied together.
  // - Flat areas appear when a positive and a negative output value are
  //   multiplied together.
  // - Ridges appear when two positive output values are multiplied together.
  let mountainousLow_mu = Multiply::new(&mountainousLow_rm0, &mountainousLow_rm1);

  // 4: [Low-mountainous-terrain subgroup]: Caches the output value from the
  // low-mountainous-terrain module.
  let mountainousLow = Cache::new(mountainousLow_mu);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: mountainous terrain (7 noise functions)
  //
  // This subgroup generates the final mountainous terrain by combining the
  // high-mountainous-terrain subgroup with the low-mountainous-terrain
  // subgroup.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Scaled-low-mountainous-terrain module]: First, this scale/bias module
  // scales the output value from the low-mountainous-terrain subgroup to a very
  // low value and biases it towards -1.0. This results in the low mountainous
  // areas becoming more-or-less flat with little variation. This will also
  // result in the low mountainous areas appearing at the lowest elevations in
  // this subgroup.
  let mountainousTerrain_sb0 = ScaleBias::new(&mountainousLow)
      .set_scale(0.03125)
      .set_bias(-0.96875);

  // 2: [Scaled-high-mountainous-terrain module]: Next, this scale/bias module
  // scales the output value from the high-mountainous-terrain subgroup to 1/4
  // of its initial value and biases it so that its output value is usually
  // positive.
  let mountainousTerrain_sb1 = ScaleBias::new(&mountainousHigh)
      .set_scale(0.25)
      .set_bias(0.25);

  // 3: [Added-high-mountainous-terrain module]: This addition module adds the
  // output value from the scaled-high-mountainous-terrain module to the
  // output value from the mountain-base-definition subgroup. Mountains now
  // appear all over the terrain.
  let mountainousTerrain_ad = Add::new(&mountainousTerrain_sb1, &mountainBaseDef);

  // 4: [Combined-mountainous-terrain module]: Note that at this point, the
  // entire terrain is covered in high mountainous terrain, even at the low
  // elevations. To make sure the mountains only appear at the higher
  // elevations, this selector module causes low mountainous terrain to appear
  // at the low elevations (within the valleys) and the high mountainous
  // terrain to appear at the high elevations (within the ridges). To do this,
  // this noise function selects the output value from the added-high-
  // mountainous-terrain module if the output value from the mountain-base-
  // definition subgroup is higher than a set amount. Otherwise, this noise
  // module selects the output value from the scaled-low-mountainous-terrain
  // module.
  let mountainousTerrain_se = Select::new(
      &mountainousTerrain_sb0,
      &mountainousTerrain_ad,
      &mountainBaseDef,
  )
  .set_bounds(-0.5, 999.5)
  .set_falloff(0.5);

  // 5: [Scaled-mountainous-terrain-module]: This scale/bias module slightly
  // reduces the range of the output value from the combined-mountainous-
  // terrain module, decreasing the heights of the mountain peaks.
  let mountainousTerrain_sb2 = ScaleBias::new(&mountainousTerrain_se)
      .set_scale(0.8)
      .set_bias(0.0);

  // 6: [Glaciated-mountainous-terrain-module]: This exponential-curve module
  // applies an exponential curve to the output value from the scaled-
  // mountainous-terrain module. This causes the slope of the mountains to
  // smoothly increase towards higher elevations, as if a glacier ground out
  // those mountains. This exponential-curve module expects the output value
  // to range from -1.0 to +1.0.
  let mountainousTerrain_ex =
      Exponent::new(&mountainousTerrain_sb2).set_exponent(MOUNTAIN_GLACIATION);

  let mountainousTerrain = Cache::new(mountainousTerrain_ex);

  // ////////////////////////////////////////////////////////////////////////
  // Function group: hilly terrain
  // ////////////////////////////////////////////////////////////////////////

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: hilly terrain (11 noise functions)
  //
  // This subgroup generates the hilly terrain.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Hills module]: This billow-noise function generates the hills.
  let hillyTerrain_bi = Billow::new()
      .set_seed(CURRENT_SEED + 60)
      .set_frequency(1663.0)
      .set_persistence(0.5)
      .set_lacunarity(HILLS_LACUNARITY)
      .set_octaves(6);

  // 2: [Scaled-hills module]: Next, a scale/bias module scales the output
  // value from the hills module so that its hilltops are not too high. The
  // reason for this is that these hills are eventually added to the river
  // valleys (see below).
  let hillyTerrain_sb0 = ScaleBias::new(&hillyTerrain_bi)
      .set_scale(0.5)
      .set_bias(0.5);

  // 3: [River-valley module]: This ridged-multifractal-noise function generates
  // the river valleys. It has a much lower frequency so that more hills will
  // appear in between the valleys. Note that this noise function generates
  // ridged-multifractal noise using only one octave; this information will be
  // important in the next step.
  let hillyTerrain_rm = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 61)
      .set_frequency(367.5)
      .set_lacunarity(HILLS_LACUNARITY)
      .set_octaves(1);

  // 4: [Scaled-river-valley module]: Next, a scale/bias module applies a
  // scaling factor of -2.0 to the output value from the river-valley module.
  // This stretches the possible elevation values because one-octave ridged-
  // multifractal noise has a lower range of output values than multiple-
  // octave ridged-multifractal noise. The negative scaling factor inverts
  // the range of the output value, turning the ridges from the river-valley
  // module into valleys.
  let hillyTerrain_sb1 = ScaleBias::new(&hillyTerrain_rm)
      .set_scale(-2.0)
      .set_bias(-1.0);

  // 5: [Low-flat module]: This low constant value is used by step 6.
  let hillyTerrain_co = Constant::new(-1.0);

  // 6: [Mountains-and-valleys module]: This blender module merges the scaled-
  // hills module and the scaled-river-valley module together. It causes the
  // low-lying areas of the terrain to become smooth, and causes the high-
  // lying areas of the terrain to contain hills. To do this, it uses uses the
  // scaled-hills module as the control module, causing the low-flat module to
  // appear in the lower areas and causing the scaled-river-valley module to
  // appear in the higher areas.
  let hillyTerrain_bl = Blend::new(&hillyTerrain_co, &hillyTerrain_sb1, &hillyTerrain_sb0);

  // 7: [Scaled-hills-and-valleys module]: This scale/bias module slightly
  // reduces the range of the output value from the hills-and-valleys
  // module, decreasing the heights of the hilltops.
  let hillyTerrain_sb2 = ScaleBias::new(&hillyTerrain_bl)
      .set_scale(0.75)
      .set_bias(-0.25);

  // 8: [Increased-slope-hilly-terrain module]: To increase the hill slopes
  // at higher elevations, this exponential-curve module applies an
  // exponential curve to the output value the scaled-hills-and-valleys
  // module. This exponential-curve module expects the input value to range
  // from -1.0 to 1.0.
  let hillyTerrain_ex = Exponent::new(&hillyTerrain_sb2).set_exponent(1.375);

  // 9: [Coarse-turbulence module]: This turbulence module warps the output
  // value from the increased-slope-hilly-terrain module, adding some
  // coarse detail to it.
  let hillyTerrain_tu0 = Turbulence::new(hillyTerrain_ex)
      .set_seed(CURRENT_SEED + 62)
      .set_frequency(1531.0)
      .set_power(1.0 / 16921.0 * HILLS_TWIST)
      .set_roughness(4);

  // 10: [Warped-hilly-terrain module]: This turbulence module warps the
  // output value from the coarse-turbulence module. This turbulence has a
  // higher frequency, but lower power, than the coarse-turbulence module,
  // adding some fine detail to it.
  let hillyTerrain_tu1 = Turbulence::new(hillyTerrain_tu0)
      .set_seed(CURRENT_SEED + 63)
      .set_frequency(21617.0)
      .set_power(1.0 / 117529.0 * HILLS_TWIST)
      .set_roughness(6);

  // 11: [Hilly-terrain group]: Caches the output value from the warped-hilly-
  // terrain module. This is the output value for the entire hilly-terrain
  // group.
  let hillyTerrain = Cache::new(hillyTerrain_tu1);

  // ////////////////////////////////////////////////////////////////////////
  // Function group: plains terrain
  // ////////////////////////////////////////////////////////////////////////

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: plains terrain (7 noise functions)
  //
  // This subgroup generates the plains terrain.
  //
  // Because this subgroup will eventually be flattened considerably, the
  // types and combinations of noise functions that generate the plains are not
  // really that important; they only need to "look" interesting.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Plains-basis-0 module]: This billow-noise function, along with the
  // plains-basis-1 module, produces the plains.
  let plainsTerrain_bi0 = Billow::new()
      .set_seed(CURRENT_SEED + 70)
      .set_frequency(1097.5)
      .set_persistence(0.5)
      .set_lacunarity(PLAINS_LACUNARITY)
      .set_octaves(8);

  // 2: [Positive-plains-basis-0 module]: This scale/bias module makes the
  // output value from the plains-basis-0 module positive since this output
  // value will be multiplied together with the positive-plains-basis-1
  // module.
  let plainsTerrain_sb0 = ScaleBias::new(&plainsTerrain_bi0)
      .set_scale(0.5)
      .set_bias(0.5);

  // 3: [Plains-basis-1 module]: This billow-noise function, along with the
  // plains-basis-2 module, produces the plains.
  let plainsTerrain_bi1 = Billow::new()
      .set_seed(CURRENT_SEED + 71)
      .set_frequency(1097.5)
      .set_persistence(0.5)
      .set_lacunarity(PLAINS_LACUNARITY)
      .set_octaves(8);

  // 4: [Positive-plains-basis-1 module]: This scale/bias module makes the
  // output value from the plains-basis-1 module positive since this output
  // value will be multiplied together with the positive-plains-basis-0
  // module.
  let plainsTerrain_sb1 = ScaleBias::new(&plainsTerrain_bi1)
      .set_scale(0.5)
      .set_bias(0.5);

  // 5: [Combined-plains-basis module]: This multiplication module combines
  // the two plains basis modules together.
  let plainsTerrain_mu = Multiply::new(&plainsTerrain_sb0, &plainsTerrain_sb1);

  // 6: [Rescaled-plains-basis module]: This scale/bias module maps the output
  // value that ranges from 0.0 to 1.0 back to a value that ranges from
  // -1.0 to +1.0.
  let plainsTerrain_sb2 = ScaleBias::new(&plainsTerrain_mu)
      .set_scale(2.0)
      .set_bias(-1.0);

  // 7: [Plains-terrain group]: Caches the output value from the rescaled-
  // plains-basis module.  This is the output value for the entire plains-
  // terrain group.
  let plainsTerrain = Cache::new(plainsTerrain_sb2);

  // ////////////////////////////////////////////////////////////////////////
  // Function group: badlands terrain
  // ////////////////////////////////////////////////////////////////////////

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: badlands sand (6 noise functions)
  //
  // This subgroup generates the sandy terrain for the badlands.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Sand-dunes module]: This ridged-multifractal-noise function generates
  // sand dunes. This ridged-multifractal noise is generated with a single
  // octave, which makes very smooth dunes.
  let badlandsSand_rm = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 80)
      .set_frequency(6163.5)
      .set_lacunarity(BADLANDS_LACUNARITY)
      .set_octaves(1);

  // 2: [Scaled-sand-dunes module]: This scale/bias module shrinks the dune
  // heights by a small amount. This is necessary so that the subsequent
  // noise functions in this subgroup can add some detail to the dunes.
  let badlandsSand_sb0 = ScaleBias::new(&badlandsSand_rm)
      .set_scale(0.875)
      .set_bias(0.0);

  // 3: [Dune-detail module]: This noise function uses Voronoi polygons to
  // generate the detail to add to the dunes. By enabling the distance
  // algorithm, small polygonal pits are generated; the edges of the pits
  // are joined to the edges of nearby pits.
  let badlandsSand_wo = Worley::new()
      .set_seed(CURRENT_SEED + 81)
      .set_frequency(16183.25)
      .set_displacement(0.0)
      .enable_range(true);

  // 4: [Scaled-dune-detail module]: This scale/bias module shrinks the dune
  // details by a large amount. This is necessary so that the subsequent
  // noise functions in this subgroup can add this detail to the sand-dunes
  // module.
  let badlandsSand_sb1 = ScaleBias::new(&badlandsSand_wo)
      .set_scale(0.25)
      .set_bias(0.25);

  // 5: [Dunes-with-detail module]: This addition module combines the scaled-
  // sand-dunes module with the scaled-dune-detail module.
  let badlandsSand_ad = Add::new(&badlandsSand_sb0, &badlandsSand_sb1);

  // 6: [Badlands-sand subgroup]: Caches the output value from the dunes-with-
  // detail module.
  let badlandsSand = Cache::new(badlandsSand_ad);

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: badlands cliffs (7 noise functions)
  //
  // This subgroup generates the cliffs for the badlands.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Cliff-basis module]: This Perlin-noise function generates some coherent
  // noise that will be used to generate the cliffs.
  let badlandsCliffs_fb = Fbm::new()
      .set_seed(CURRENT_SEED + 90)
      .set_frequency(CONTINENT_FREQUENCY * 839.0)
      .set_persistence(0.5)
      .set_lacunarity(BADLANDS_LACUNARITY)
      .set_octaves(6);

  // 2: [Cliff-shaping module]: Next, this curve module applies a curve to
  // the output value from the cliff-basis module. This curve is initially
  // very shallow, but then its slope increases sharply. At the highest
  // elevations, the curve becomes very flat again. This produces the
  // stereotypical Utah-style desert cliffs.
  let badlandsCliffs_cu = Curve::new(&badlandsCliffs_fb)
      .add_control_point(-2.000, -2.000)
      .add_control_point(-1.000, -1.000)
      .add_control_point(-0.000, -0.750)
      .add_control_point(0.500, -0.250)
      .add_control_point(0.625, 0.875)
      .add_control_point(0.750, 1.000)
      .add_control_point(2.000, 1.250);

  // 3: [Clamped-cliffs module]: This clamping module makes the tops of the
  // cliffs very flat by clamping the output value from the cliff-shaping
  // module.
  let badlandsCliffs_cl = Clamp::new(&badlandsCliffs_cu).set_bounds(-999.125, 0.875);

  // 4: [Terraced-cliffs module]: Next, this terracing module applies some
  // terraces to the clamped-cliffs module in the lower elevations before the
  // sharp cliff transition.
  let badlandsCliffs_te = Terrace::new(&badlandsCliffs_cl)
      .add_control_point(-1.000)
      .add_control_point(-0.875)
      .add_control_point(-0.750)
      .add_control_point(-0.500)
      .add_control_point(0.000)
      .add_control_point(1.000);

  // 5: [Coarse-turbulence module]: This turbulence module warps the output
  // value from the terraced-cliffs module, adding some coarse detail to it.
  let badlandsCliffs_tu0 = Turbulence::new(badlandsCliffs_te)
      .set_seed(CURRENT_SEED + 91)
      .set_frequency(16111.0)
      .set_power(1.0 / 141539.0 * BADLANDS_TWIST)
      .set_roughness(3);

  // 6: [Warped-cliffs module]: This turbulence module warps the output value
  // from the coarse-turbulence module. This turbulence has a higher
  // frequency, but lower power, than the coarse-turbulence module, adding
  // some fine detail to it.
  let badlandsCliffs_tu1 = Turbulence::new(badlandsCliffs_tu0)
      .set_seed(CURRENT_SEED + 92)
      .set_frequency(36107.0)
      .set_power(1.0 / 211543.0 * BADLANDS_TWIST)
      .set_roughness(3);

  // 7: [Badlands-cliffs subgroup]: Caches the output value from the warped-
  // cliffs module.
  let badlandsCliffs = Cache::new(badlandsCliffs_tu1);

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: badlands terrain (3 noise functions)
  //
  // Generates the final badlands terrain.
  //
  // Using a scale/bias module, the badlands sand is flattened considerably,
  // then the sand elevations are lowered to around -1.0. The maximum value
  // from the flattened sand module and the cliff module contributes to the
  // final elevation. This causes sand to appear at the low elevations since
  // the sand is slightly higher than the cliff base.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Scaled-sand-dunes module]: This scale/bias module considerably
  // flattens the output value from the badlands-sands subgroup and lowers
  // this value to near -1.0.
  let badlandsTerrain_sb = ScaleBias::new(&badlandsSand)
      .set_scale(0.25)
      .set_bias(-0.75);

  // 2: [Dunes-and-cliffs module]: This maximum-value module causes the dunes
  // to appear in the low areas and the cliffs to appear in the high areas.
  // It does this by selecting the maximum of the output values from the
  // scaled-sand-dunes module and the badlands-cliffs subgroup.
  let badlandsTerrain_ma = Max::new(&badlandsCliffs, &badlandsTerrain_sb);

  // 3: [Badlands-terrain group]: Caches the output value from the dunes-and-
  // cliffs module. This is the output value for the entire badlands-terrain
  // group.
  let badlandsTerrain = Cache::new(badlandsTerrain_ma);

  //    debug::render_noise_module("complexplanet_images/12_2_badlandsTerrain.png",
  //                               &badlandsTerrain,
  //                               1024,
  //                               1024,
  //                               1000);

  // ////////////////////////////////////////////////////////////////////////
  // Function group: river positions
  // ////////////////////////////////////////////////////////////////////////

  // ////////////////////////////////////////////////////////////////////////
  // Function subgroup: river positions (7 noise functions)
  //
  // This subgroup generates the river positions.
  //
  // -1.0 represents the lowest elevations and +1.0 represents the highest
  // elevations.
  //

  // 1: [Large-river-basis module]: This ridged-multifractal-noise function
  // creates the large, deep rivers.
  let riverPositions_rm0 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 100)
      .set_frequency(18.75)
      .set_lacunarity(CONTINENT_LACUNARITY)
      .set_octaves(1);

  // 2: [Large-river-curve module]: This curve module applies a curve to the
  // output value from the large-river-basis module so that the ridges become
  // inverted. This creates the rivers. This curve also compresses the edge of
  // the rivers, producing a sharp transition from the land to the river
  // bottom.
  let riverPositions_cu0 = Curve::new(&riverPositions_rm0)
      .add_control_point(-2.000, 2.000)
      .add_control_point(-1.000, 1.000)
      .add_control_point(-0.125, 0.875)
      .add_control_point(0.000, -1.000)
      .add_control_point(1.000, -1.500)
      .add_control_point(2.000, -2.000);

  // 3: [Small-river-basis module]: This ridged-multifractal-noise function
  // creates the small, shallow rivers.
  let riverPositions_rm1 = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 101)
      .set_frequency(43.25)
      .set_lacunarity(CONTINENT_LACUNARITY)
      .set_octaves(1);

  // 4: [Small-river-curve module]: This curve module applies a curve to the
  // output value from the small-river-basis module so that the ridges become
  // inverted. This creates the rivers. This curve also compresses the edge of
  // the rivers, producing a sharp transition from the land to the river
  // bottom.
  let riverPositions_cu1 = Curve::new(&riverPositions_rm1)
      .add_control_point(-2.000, 2.0000)
      .add_control_point(-1.000, 1.5000)
      .add_control_point(-0.125, 1.4375)
      .add_control_point(0.000, 0.5000)
      .add_control_point(1.000, 0.2500)
      .add_control_point(2.000, 0.0000);

  // 5: [Combined-rivers module]: This minimum-value module causes the small
  // rivers to cut into the large rivers.  It does this by selecting the
  // minimum output values from the large-river-curve module and the small-
  // river-curve module.
  let riverPositions_mi = Min::new(&riverPositions_cu0, &riverPositions_cu1);

  // 6: [Warped-rivers module]: This turbulence module warps the output value
  //    from the combined-rivers module, which twists the rivers.  The high
  //    roughness produces less-smooth rivers.
  let riverPositions_tu = Turbulence::new(riverPositions_mi)
      .set_seed(CURRENT_SEED + 102)
      .set_frequency(9.25)
      .set_power(1.0 / 57.75)
      .set_roughness(6);

  // 7: [River-positions group]: Caches the output value from the warped-
  //    rivers module.  This is the output value for the entire river-
  //    positions group.
  let riverPositions = Cache::new(riverPositions_tu);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: scaled mountainous terrain
  // /////////////////////////////////////////////////////////////////////////

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: scaled mountainous terrain (6 noise functions)
  //
  // This subgroup scales the output value from the mountainous-terrain group
  // so that it can be added to the elevation defined by the continent-
  // definition group.
  //
  // This subgroup scales the output value such that it is almost always
  // positive.  This is done so that a negative elevation does not get applied
  // to the continent-definition group, preventing parts of that group from
  // having negative terrain features "stamped" into it.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Base-scaled-mountainous-terrain module]: This scale/bias module
  // scales the output value from the mountainous-terrain group so that the
  // output value is measured in planetary elevation units.
  let scaledMountainousTerrain_sb0 = ScaleBias::new(&mountainousTerrain)
      .set_scale(0.125)
      .set_bias(0.125);

  // 2: [Base-peak-modulation module]: At this stage, most mountain peaks have
  // roughly the same elevation. This BasicMulti module generates some
  // random values that will be used by subsequent noise functions to randomly
  // change the elevations of the mountain peaks.
  let scaledMountainousTerrain_fb = Fbm::new()
      .set_seed(CURRENT_SEED + 110)
      .set_frequency(14.5)
      .set_persistence(0.5)
      .set_lacunarity(MOUNTAIN_LACUNARITY)
      .set_octaves(6);

  // 3: [Peak-modulation module]: This exponential-curve module applies an
  // exponential curve to the output value from the base-peak-modulation
  // module. This produces a small number of high values and a much larger
  // number of low values. This means there will be a few peaks with much
  // higher elevations than the majority of the peaks, making the terrain
  // features more varied.
  let scaledMountainousTerrain_ex =
      Exponent::new(&scaledMountainousTerrain_fb).set_exponent(1.25);

  // 4: [Scaled-peak-modulation module]: This scale/bias module modifies the
  // range of the output value from the peak-modulation module so that it can
  // be used as the modulator for the peak-height-multiplier module. It is
  // important that this output value is not much lower than 1.0.
  let scaledMountainousTerrain_sb1 = ScaleBias::new(&scaledMountainousTerrain_ex)
      .set_scale(0.25)
      .set_bias(1.0);

  // 5: [Peak-height-multiplier module]: This multiplier module modulates the
  // heights of the mountain peaks from the base-scaled-mountainous-terrain
  // module using the output value from the scaled-peak-modulation module.
  let scaledMountainousTerrain_mu =
      Multiply::new(&scaledMountainousTerrain_sb0, &scaledMountainousTerrain_sb1);

  // 6: [Scaled-mountainous-terrain group]: Caches the output value from the
  // peak-height-multiplier module.  This is the output value for the
  // entire scaled-mountainous-terrain group.
  let scaledMountainousTerrain = Cache::new(scaledMountainousTerrain_mu);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: scaled hilly terrain
  // /////////////////////////////////////////////////////////////////////////

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: scaled hilly terrain (6 noise functions)
  //
  // This subgroup scales the output value from the hilly-terrain group so
  // that it can be added to the elevation defined by the continent-
  // definition group. The scaling amount applied to the hills is one half of
  // the scaling amount applied to the scaled-mountainous-terrain group.
  //
  // This subgroup scales the output value such that it is almost always
  // positive. This is done so that negative elevations are not applied to
  // the continent-definition group, preventing parts of the continent-
  // definition group from having negative terrain features "stamped" into it.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Base-scaled-hilly-terrain module]: This scale/bias module scales the
  // output value from the hilly-terrain group so that this output value is
  // measured in planetary elevation units.
  let scaledHillyTerrain_sb0 = ScaleBias::new(&hillyTerrain)
      .set_scale(0.0625)
      .set_bias(0.0625);

  // 2: [Base-hilltop-modulation module]: At this stage, most hilltops have
  // roughly the same elevation. This BasicMulti module generates some
  // random values that will be used by subsequent noise functions to
  // randomly change the elevations of the hilltops.
  let scaledHillyTerrain_fb = Fbm::new()
      .set_seed(CURRENT_SEED + 120)
      .set_frequency(13.5)
      .set_persistence(0.5)
      .set_lacunarity(HILLS_LACUNARITY)
      .set_octaves(6);

  // 3: [Hilltop-modulation module]: This exponential-curve module applies an
  // exponential curve to the output value from the base-hilltop-modulation
  // module. This produces a small number of high values and a much larger
  // number of low values. This means there will be a few hilltops with
  // much higher elevations than the majority of the hilltops, making the
  // terrain features more varied.
  let scaledHillyTerrain_ex = Exponent::new(&scaledHillyTerrain_fb).set_exponent(1.25);

  // 4: [Scaled-hilltop-modulation module]: This scale/bias module modifies
  // the range of the output value from the hilltop-modulation module so that
  // it can be used as the modulator for the hilltop-height-multiplier module.
  // It is important that this output value is not much lower than 1.0.
  let scaledHillyTerrain_sb1 = ScaleBias::new(&scaledHillyTerrain_ex)
      .set_scale(0.5)
      .set_bias(1.5);

  // 5: [Hilltop-height-multiplier module]: This multiplier module modulates
  // the heights of the hilltops from the base-scaled-hilly-terrain module
  // using the output value from the scaled-hilltop-modulation module.
  let scaledHillyTerrain_mu = Multiply::new(&scaledHillyTerrain_sb0, &scaledHillyTerrain_sb1);

  // 6: [Scaled-hilly-terrain group]: Caches the output value from the
  // hilltop-height-multiplier module. This is the output value for the entire
  // scaled-hilly-terrain group.
  let scaledHillyTerrain = Cache::new(scaledHillyTerrain_mu);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: scaled plains terrain
  // /////////////////////////////////////////////////////////////////////////

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: scaled plains terrain (2 noise functions)
  //
  // This subgroup scales the output value from the plains-terrain group so
  // that it can be added to the elevations defined by the continent-
  // definition group.
  //
  // This subgroup scales the output value such that it is almost always
  // positive. This is done so that negative elevations are not applied to
  // the continent-definition group, preventing parts of the continent-
  // definition group from having negative terrain features "stamped" into it.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Scaled-plains-terrain module]: This scale/bias module greatly
  // flattens the output value from the plains terrain.  This output value
  // is measured in planetary elevation units.
  let scaledPlainsTerrain_sb0 = ScaleBias::new(&plainsTerrain)
      .set_scale(0.00390625)
      .set_bias(0.0078125);

  // 2: [Scaled-plains-terrain group]: Caches the output value from the
  // scaled-plains-terrain module. This is the output value for the entire
  // scaled-plains-terrain group.
  let scaledPlainsTerrain = Cache::new(scaledPlainsTerrain_sb0);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: scaled badlands terrain
  // /////////////////////////////////////////////////////////////////////////

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: scaled badlands terrain (2 noise functions)
  //
  // This subgroup scales the output value from the badlands-terrain group so
  // that it can be added to the elevations defined by the continent-
  // definition group.
  //
  // This subgroup scales the output value such that it is almost always
  // positive. This is done so that negative elevations are not applied to the
  // continent-definition group, preventing parts of the continent-definition
  // group from having negative terrain features "stamped" into it.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Scaled-badlands-terrain module]: This scale/bias module scales the
  // output value from the badlands-terrain group so that it is measured
  // in planetary elevation units.
  let scaledBadlandsTerrain_sb = ScaleBias::new(&badlandsTerrain)
      .set_scale(0.0625)
      .set_bias(0.0625);

  // 2: [Scaled-badlands-terrain group]: Caches the output value from the
  // scaled-badlands-terrain module. This is the output value for the
  // entire scaled-badlands-terrain group.
  let scaledBadlandsTerrain = Cache::new(scaledBadlandsTerrain_sb);

  //    debug::render_noise_module("complexplanet_images/17_0_scaledBadlandsTerrain\
  //    .png",
  //                               &scaledBadlandsTerrain,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: final planet
  // /////////////////////////////////////////////////////////////////////////

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: continental shelf (6 noise functions)
  //
  // This module subgroup creates the continental shelves.
  //
  // The output value from this module subgroup are measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Shelf-creator module]: This terracing module applies a terracing
  // curve to the continent-definition group at the specified shelf level.
  // This terrace becomes the continental shelf. Note that this terracing
  // module also places another terrace below the continental shelf near -1.0.
  // The bottom of this terrace is defined as the bottom of the ocean;
  // subsequent noise functions will later add oceanic trenches to the bottom of
  // the ocean.
  let continentalShelf_te = Terrace::new(&continentDef)
      .add_control_point(-1.0)
      .add_control_point(-0.75)
      .add_control_point(SHELF_LEVEL)
      .add_control_point(1.0);

  //    debug::render_noise_module("complexplanet_images/18_0_continentalShelf_te\
  //    .png",
  //                               &continentalShelf_te,
  //                               1024,
  //                               1024,
  //                               1000);

  // 2: [Clamped-sea-bottom module]: This clamping module clamps the output
  // value from the shelf-creator module so that its possible range is from
  // the bottom of the ocean to sea level. This is done because this subgroup
  // is only concerned about the oceans.
  let continentalShelf_cl = Clamp::new(&continentalShelf_te).set_bounds(-0.75, SEA_LEVEL);

  //    debug::render_noise_module("complexplanet_images/18_1_continentalShelf_cl\
  //    .png",
  //                               &continentalShelf_cl,
  //                               1024,
  //                               1024,
  //                               1000);

  // 3: [Oceanic-trench-basis module]: This ridged-multifractal-noise function
  // generates some coherent noise that will be used to generate the oceanic
  // trenches. The ridges represent the bottom of the trenches.
  let continentalShelf_rm = RidgedMulti::new()
      .set_seed(CURRENT_SEED + 130)
      .set_frequency(CONTINENT_FREQUENCY * 4.375)
      .set_lacunarity(CONTINENT_LACUNARITY)
      .set_octaves(16);

  //    debug::render_noise_module("complexplanet_images/18_2_continentalShelf_rm\
  //    .png",
  //                               &continentalShelf_rm,
  //                               1024,
  //                               1024,
  //                               1000);

  // 4: [Oceanic-trench module]: This scale/bias module inverts the ridges
  // from the oceanic-trench-basis-module so that the ridges become trenches.
  // This noise function also reduces the depth of the trenches so that their
  // depths are measured in planetary elevation units.
  let continentalShelf_sb = ScaleBias::new(&continentalShelf_rm)
      .set_scale(-0.125)
      .set_bias(-0.125);

  //    debug::render_noise_module("complexplanet_images/18_3_continentalShelf_sb\
  //    .png",
  //                               &continentalShelf_sb,
  //                               1024,
  //                               1024,
  //                               1000);

  // 5: [Shelf-and-trenches module]: This addition module adds the oceanic
  // trenches to the clamped-sea-bottom module.
  let continentalShelf_ad = Add::new(&continentalShelf_sb, &continentalShelf_cl);

  // 6: [Continental-shelf subgroup]: Caches the output value from the shelf-
  //    and-trenches module.
  let continentalShelf = Cache::new(continentalShelf_ad);

  //    debug::render_noise_module("complexplanet_images/18_4_continentalShelf.png",
  //                               &continentalShelf,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function group: base continent elevations (3 noise functions)
  //
  // This subgroup generates the base elevations for the continents, before
  // terrain features are added.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Base-scaled-continent-elevations module]: This scale/bias module
  // scales the output value from the continent-definition group so that it
  // is measured in planetary elevation units.
  let baseContinentElev_sb = ScaleBias::new(&continentDef)
      .set_scale(CONTINENT_HEIGHT_SCALE)
      .set_bias(0.0);

  //    debug::render_noise_module("complexplanet_images/19_0_baseContinentElev_sb\
  //    .png",
  //                               &baseContinentElev_sb,
  //                               1024,
  //                               1024,
  //                               1000);

  // 2: [Base-continent-with-oceans module]: This selector module applies the
  // elevations of the continental shelves to the base elevations of the
  // continent. It does this by selecting the output value from the
  // continental-shelf subgroup if the corresponding output value from the
  // continent-definition group is below the shelf level. Otherwise, it
  // selects the output value from the base-scaled-continent-elevations
  // module.
  let baseContinentElev_se = Select::new(&baseContinentElev_sb, &continentalShelf, &continentDef)
      .set_bounds(SHELF_LEVEL - 1000.0, SHELF_LEVEL)
      .set_falloff(0.03125);

  // 3: [Base-continent-elevation subgroup]: Caches the output value from the
  // base-continent-with-oceans module.
  let baseContinentElev = Cache::new(baseContinentElev_se);

  //    debug::render_noise_module("complexplanet_images/19_1_baseContinentElev\
  //    .png",
  //                               &baseContinentElev,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: continents with plains (2 noise functions)
  //
  // This subgroup applies the scaled-plains-terrain group to the base-
  // continent-elevation subgroup.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Continents-with-plains module]: This addition module adds the scaled-
  // plains-terrain group to the base-continent-elevation subgroup.
  let continentsWithPlains_ad = Add::new(&baseContinentElev, &scaledPlainsTerrain);

  // 2: [Continents-with-plains subgroup]: Caches the output value from the
  // continents-with-plains module.
  let continentsWithPlains = Cache::new(continentsWithPlains_ad);

  //    debug::render_noise_module("complexplanet_images/20_0_continentsWithPlains\
  //    .png",
  //                               &continentsWithPlains,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: continents with hills (3 noise functions)
  //
  // This subgroup applies the scaled-hilly-terrain group to the continents-
  // with-plains subgroup.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Continents-with-hills module]: This addition module adds the scaled-
  // hilly-terrain group to the base-continent-elevation subgroup.
  let continentsWithHills_ad = Add::new(&baseContinentElev, &scaledHillyTerrain);

  //    debug::render_noise_module("complexplanet_images/21_0_continentsWithHills_ad.png",
  //                               &continentsWithHills_ad,
  //                               1024,
  //                               1024,
  //                               1000);

  // 2: [Select-high-elevations module]: This selector module ensures that the
  // hills only appear at higher elevations. It does this by selecting the
  // output value from the continent-with-hills module if the corresponding
  // output value from the terrain-type-definition group is above a certain
  // value. Otherwise, it selects the output value from the continents-with-
  // plains subgroup.
  let continentsWithHills_se = Select::new(
      &continentsWithPlains,
      &continentsWithHills_ad,
      &terrainTypeDef,
  )
  .set_bounds(1.0 - HILLS_AMOUNT, 1001.0 - HILLS_AMOUNT)
  .set_falloff(0.25);

  // 3: [Continents-with-hills subgroup]: Caches the output value from the
  // select-high-elevations module.
  let continentsWithHills = Cache::new(continentsWithHills_se);

  //    debug::render_noise_module("complexplanet_images/21_1_continentsWithHills\
  //    .png",
  //                               &continentsWithHills,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: continents with mountains (5 noise functions)
  //
  // This subgroup applies the scaled-mountainous-terrain group to the
  // continents-with-hills subgroup.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Continents-and-mountains module]: This addition module adds the
  // scaled-mountainous-terrain group to the base-continent-elevation
  // subgroup.
  let continentsWithMountains_ad0 = Add::new(&baseContinentElev, &scaledMountainousTerrain);

  //    debug::render_noise_module("complexplanet_images/22_0_continentsWithMountains_ad0.png",
  //                               &continentsWithMountains_ad0,
  //                               1024,
  //                               1024,
  //                               1000);

  // 2: [Increase-mountain-heights module]: This curve module applies a curve
  // to the output value from the continent-definition group. This modified
  // output value is used by a subsequent noise function to add additional
  // height to the mountains based on the current continent elevation. The
  // higher the continent elevation, the higher the mountains.
  let continentsWithMountains_cu = Curve::new(&continentDef)
      .add_control_point(-1.0, -0.0625)
      .add_control_point(0.0, 0.0000)
      .add_control_point(1.0 - MOUNTAINS_AMOUNT, 0.0625)
      .add_control_point(1.0, 0.2500);

  //    debug::render_noise_module("complexplanet_images/22_1_continentsWithMountains_cu.png",
  //                               &continentsWithMountains_cu,
  //                               1024,
  //                               1024,
  //                               1000);

  // 3: [Add-increased-mountain-heights module]: This addition module adds the
  // increased-mountain-heights module to the continents-and-mountains module.
  // The highest continent elevations now have the highest mountains.
  let continentsWithMountains_ad1 =
      Add::new(&continentsWithMountains_ad0, &continentsWithMountains_cu);

  //    debug::render_noise_module("complexplanet_images/22_2_continentsWithMountains_ad1.png",
  //                               &continentsWithMountains_ad1,
  //                               1024,
  //                               1024,
  //                               1000);

  // 4: [Select-high-elevations module]: This selector module ensures that
  // mountains only appear at higher elevations. It does this by selecting the
  // output value from the continent-with-mountains module if the
  // corresponding output value from the terrain-type-definition group is
  // above a certain value. Otherwise, it selects the output value from the
  // continents-with-hills subgroup. Note that the continents-with-hills
  // subgroup also contains the plains terrain.
  let continentsWithMountains_se = Select::new(
      &continentsWithHills,
      &continentsWithMountains_ad1,
      &terrainTypeDef,
  )
  .set_bounds(1.0 - MOUNTAINS_AMOUNT, 1001.0 - MOUNTAINS_AMOUNT)
  .set_falloff(0.25);

  // 5: [Continents-with-mountains subgroup]: Caches the output value from the
  // select-high-elevations module.
  let continentsWithMountains = Cache::new(continentsWithMountains_se);

  //    debug::render_noise_module("complexplanet_images/22_3_continentsWithMountains.png",
  //                               &continentsWithMountains,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: continents with badlands (5 noise functions)
  //
  // This subgroup applies the scaled-badlands-terrain group to the
  // continents-with-mountains subgroup.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Badlands-positions module]: This BasicMulti module generates some
  // random noise, which is used by subsequent noise functions to specify the
  // locations of the badlands.
  let continentsWithBadlands_bm = Fbm::new()
      .set_seed(CURRENT_SEED + 140)
      .set_frequency(16.5)
      .set_persistence(0.5)
      .set_lacunarity(CONTINENT_LACUNARITY)
      .set_octaves(2);

  //    debug::render_noise_module("complexplanet_images/23_0_continentsWithBadlands_bm.png",
  //                               &continentsWithBadlands_bm,
  //                               1024,
  //                               1024,
  //                               1000);

  // 2: [Continents-and-badlands module]:  This addition module adds the
  // scaled-badlands-terrain group to the base-continent-elevation
  // subgroup.
  let continentsWithBadlands_ad = Add::new(&baseContinentElev, &scaledBadlandsTerrain);

  //    debug::render_noise_module("complexplanet_images/23_1_continentsWithBadlands_ad.png",
  //                               &continentsWithBadlands_ad,
  //                               1024,
  //                               1024,
  //                               1000);

  // 3: [Select-badlands-positions module]: This selector module places
  // badlands at random spots on the continents based on the BasicMulti noise
  // generated by the badlands-positions module. To do this, it selects the
  // output value from the continents-and-badlands module if the corresponding
  // output value from the badlands-position module is greater than a
  // specified value. Otherwise, this selector module selects the output value
  // from the continents-with-mountains subgroup. There is also a wide
  // transition between these two noise functions so that the badlands can blend
  // into the rest of the terrain on the continents.
  let continentsWithBadlands_se = Select::new(
      &continentsWithMountains,
      &continentsWithBadlands_ad,
      &continentsWithBadlands_bm,
  )
  .set_bounds(1.0 - BADLANDS_AMOUNT, 1001.0 - BADLANDS_AMOUNT)
  .set_falloff(0.25);

  //    debug::render_noise_module("complexplanet_images/23_2_continentsWithBadlands_se.png",
  //                               &continentsWithBadlands_se,
  //                               1024,
  //                               1024,
  //                               1000);

  // 4: [Apply-badlands module]: This maximum-value module causes the badlands
  // to "poke out" from the rest of the terrain. It does this by ensuring
  // that only the maximum of the output values from the continents-with-
  // mountains subgroup and the select-badlands-positions modules contribute
  // to the output value of this subgroup. One side effect of this process is
  // that the badlands will not appear in mountainous terrain.
  let continentsWithBadlands_ma = Max::new(&continentsWithMountains, &continentsWithBadlands_se);

  // 5: [Continents-with-badlands subgroup]: Caches the output value from the
  //    apply-badlands module.
  let continentsWithBadlands = Cache::new(continentsWithBadlands_ma);

  //    debug::render_noise_module("complexplanet_images/23_3_continentsWithBadlands.png",
  //                               &continentsWithBadlands,
  //                               1024,
  //                               1024,
  //                               1000);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: continents with rivers (4 noise functions)
  //
  // This subgroup applies the river-positions group to the continents-with-
  // badlands subgroup.
  //
  // The output value from this module subgroup is measured in planetary
  // elevation units (-1.0 for the lowest underwater trenches and +1.0 for the
  // highest mountain peaks.)
  //

  // 1: [Scaled-rivers module]: This scale/bias module scales the output value
  // from the river-positions group so that it is measured in planetary
  // elevation units and is negative; this is required for step 2.
  let continentsWithRivers_sb = ScaleBias::new(&riverPositions)
      .set_scale(RIVER_DEPTH / 2.0)
      .set_bias(-RIVER_DEPTH / 2.0);

  //    debug::render_noise_module("complexplanet_images/24_0_continentsWithRivers_sb.png",
  //                               &continentsWithRivers_sb,
  //                               1024,
  //                               1024,
  //                               1000);

  // 2: [Add-rivers-to-continents module]: This addition module adds the
  // rivers to the continents-with-badlands subgroup. Because the scaled-
  // rivers module only outputs a negative value, the scaled-rivers module
  // carves the rivers out of the terrain.
  let continentsWithRivers_ad = Add::new(&continentsWithBadlands, &continentsWithRivers_sb);

  //    debug::render_noise_module("complexplanet_images/24_1_continentsWithRivers_ad.png",
  //                               &continentsWithRivers_ad,
  //                               1024,
  //                               1024,
  //                               1000);

  // 3: [Blended-rivers-to-continents module]: This selector module outputs
  // deep rivers near sea level and shallower rivers in higher terrain.  It
  // does this by selecting the output value from the continents-with-
  // badlands subgroup if the corresponding output value from the
  // continents-with-badlands subgroup is far from sea level.  Otherwise,
  // this selector module selects the output value from the add-rivers-to-
  // continents module.
  let continentsWithRivers_se = Select::new(
      &continentsWithBadlands,
      &continentsWithRivers_ad,
      &continentsWithBadlands,
  )
  .set_bounds(SEA_LEVEL, CONTINENT_HEIGHT_SCALE + SEA_LEVEL)
  .set_falloff(CONTINENT_HEIGHT_SCALE - SEA_LEVEL);

  // 4: [Continents-with-rivers subgroup]: Caches the output value from the
  // blended-rivers-to-continents module.
  let continentsWithRivers = Cache::new(continentsWithRivers_se);

  // /////////////////////////////////////////////////////////////////////////
  // Function subgroup: unscaled final planet (1 noise function)
  //
  // This subgroup simply caches the output value from the continent-with-
  // rivers subgroup to contribute to the final output value.
  //

  // 1: [Unscaled-final-planet subgroup]: Caches the output value from the
  //    continent-with-rivers subgroup.
  let unscaledFinalPlanet = Cache::new(continentsWithRivers);

  let noise_map = PlaneMapBuilder::new(&unscaledFinalPlanet)
    .set_size(width as usize, height as usize)
    .set_x_bounds(-2.0, 2.0)
    .set_y_bounds(-2.0, 2.0)
    .build();

  points
    .iter()
    .map(|point| noise_map.get_value(point.0 as usize, point.1 as usize))
    .collect::<Vec<f64>>()

  // This does not work, I'm probably not understanding value generation
  // points
  //   .iter()
  //   .map(|point| unscaledFinalPlanet.get([point.0, point.1, 0.0]))
  //   .collect::<Vec<f64>>()
}
