// Solace — Toronto LST Export
// Exports mean summer Landsat 8/9 surface temperature
// clipped to Toronto, 2015-2024, as GeoTIFF to Google Drive

var toronto = ee.Geometry.Rectangle([-79.65, 43.55, -79.10, 43.90]);

var getLST = function(year) {
  var start = ee.Date.fromYMD(year, 6, 1);
  var end = ee.Date.fromYMD(year, 8, 31);
  
  var collection = ee.ImageCollection('LANDSAT/LC08/C02/T1_L2')
    .merge(ee.ImageCollection('LANDSAT/LC09/C02/T1_L2'))
    .filterBounds(toronto)
    .filterDate(start, end)
    .filter(ee.Filter.lt('CLOUD_COVER', 20));
  
  var meanLST = collection.select('ST_B10').mean()
    .multiply(0.00341802)
    .add(149.0)
    .subtract(273.15)
    .rename('LST_celsius');
  
  return meanLST.set('year', year);
};

// Export one image per year
var years = [2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024];

years.forEach(function(year) {
  var img = getLST(year);
  
  Export.image.toDrive({
    image: img,
    description: 'toronto_lst_' + year,
    folder: 'solace',
    fileNamePrefix: 'toronto_lst_' + year,
    region: toronto,
    scale: 30,
    crs: 'EPSG:4326',
    fileFormat: 'GeoTIFF',
    maxPixels: 1e9
  });
});

// Solace — Toronto Land Cover Export
// Solace — Toronto Land Cover Export (ESA WorldCover)

var toronto = ee.Geometry.Rectangle([-79.65, 43.55, -79.10, 43.90]);

var worldcover = ee.ImageCollection('ESA/WorldCover/v200')
  .first()
  .select('Map');

Export.image.toDrive({
  image: worldcover,
  description: 'toronto_landcover',
  folder: 'solace',
  fileNamePrefix: 'toronto_landcover',
  region: toronto,
  scale: 10,
  crs: 'EPSG:4326',
  fileFormat: 'GeoTIFF',
  maxPixels: 1e9
});