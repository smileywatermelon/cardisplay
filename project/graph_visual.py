import pandas as pd
import matplotlib.pyplot as plt

csv_filename = "hp_torque_curve.csv"
df = pd.read_csv(csv_filename)

fig, ax1 = plt.subplots()

ax1.plot(df["RPM"], df["Torque (ft-lb)"], color='orange', label="Torque (ft-lb)")
ax1.set_xlabel("RPM")
ax1.set_ylabel("Torque (ft-lb)", color='black')
ax1.tick_params(axis='y', labelcolor='black')

ax2 = ax1.twinx()
ax2.plot(df["RPM"], df["Horsepower (hp)"], color='blue', label="Horsepower (hp)")
ax2.set_ylabel("Horsepower (hp)", color='white')
ax2.tick_params(axis='y', labelcolor='white')

fig.suptitle("Horsepower and Torque Curve")
ax1.grid(True)

plt.show()
