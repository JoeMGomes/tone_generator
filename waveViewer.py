import pandas as pd

import matplotlib.pyplot as plot

df = pd.read_csv('output.csv',header=None)
print(df.head())
df.shape

plot.figure(figsize=(20,10))
plot.plot(df[0:24000])
plot.show()