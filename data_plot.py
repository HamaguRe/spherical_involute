import csv
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

# CSVファイルを読んでヘッダと値を対応させた辞書型を返す
#
# --- CSV File ---
# x,y,z
# 0,0.01,-0.1
# 1,0.02,-0.2
# 2,0.03,-0.3
# ---- Return -----
# {'x': [0,1,2], 'y': [0.01,0.02,0.03], 'z': [-0.1,-0.2,-0.3]}
def load_csv(file_path):
    tooth_surface_dict = {}
    with open(file_path, 'r', newline='', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            for key, value in row.items():
                if key not in tooth_surface_dict:
                    tooth_surface_dict[key] = []
                tooth_surface_dict[key].append(float(value))

    return tooth_surface_dict

# Figureを追加
fig = plt.figure(figsize = (8, 8))

# 3DAxesを追加
ax = fig.add_subplot(111, projection='3d')

# Axesのタイトルを設定
ax.set_title("Spherical involute curve", size = 20)

# 軸ラベルを設定
ax.set_xlabel("x", size = 14)
ax.set_ylabel("y", size = 14)
ax.set_zlabel("z", size = 14)

# 軸の範囲を設定
axis_range = 20
ax.set_xlim(-axis_range, axis_range)
ax.set_ylim(-axis_range, axis_range)
ax.set_zlim(-axis_range, 0)

# ---- 円錐を表示 ----
base_cone_surface = load_csv("./base_cone_surface.csv")
x = np.array(base_cone_surface['x'])
y = np.array(base_cone_surface['y'])
z = np.array(base_cone_surface['z'])

first_z = z[0]
theta_split = 1
for i in range(1, len(z)):
    if first_z == z[i]:
        theta_split += 1
    else:
        break

gen_split = len(z) // theta_split

# 円錐座標の計算をthetaとgeneratrixのくくりでやってるので、
# メッシュプロットするためにそれらをまとめる必要がある。
X = x.reshape(gen_split, theta_split)
Y = y.reshape(gen_split, theta_split)
Z = z.reshape(gen_split, theta_split)
ax.plot_surface(X, Y, Z, color='c', alpha=0.3, edgecolor='k', label="Base cone")
# -----------------------

# --- 球面インボリュートヘリコイドを表示 ---
tooth_surface = load_csv("./tooth_surface.csv")
x = np.array(tooth_surface['x'])
y = np.array(tooth_surface['y'])
z = np.array(tooth_surface['z'])

ax.plot(x, y, z, label="Spherical involute curve", color="green")

plt.legend()
plt.show()