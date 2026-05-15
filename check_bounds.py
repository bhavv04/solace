from osgeo import gdal
from pathlib import Path

hdf_files = sorted(Path('data/raw/modis').rglob('*.hdf'))
f = str(hdf_files[0])
ds = gdal.Open(f'HDF4_EOS:EOS_GRID:\"{f}\":MODIS_Grid_Daily_1km_LST:LST_Day_1km')
gt = ds.GetGeoTransform()
width = ds.RasterXSize
height = ds.RasterYSize
print('Left:', gt[0])
print('Top:', gt[3])
print('Right:', gt[0] + width * gt[1])
print('Bottom:', gt[3] + height * gt[5])
print('Projection:', ds.GetProjection()[:150])
