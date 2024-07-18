import os,json
import numpy as np
import scipy.integrate as sci
import random, math
import matplotlib
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

from geomdl import NURBS
from geomdl import helpers
from geomdl import utilities
from geomdl import operations
# Import Matplotlib visualization module
# from geomdl.visualization import VisMPL


file_name = 'bsurface-tests.json'
pmax = 5
qmax = 6
orders = [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6]]

knots_u = [0, 0, 0, 0, 0, 0, 1 / 14, 3 / 14, 4 / 14, 6 / 14, 7 / 14, 9 / 14, 1, 1, 1, 1, 1, 1]
m_max = len(knots_u)
r_max = m_max - 1 - pmax
knots_v = [0, 0, 0, 0, 0, 0, 0, 1 / 14, 2 / 14, 4.5 / 14, 5.2 / 14, 7 / 14,  9 / 14, 1, 1, 1, 1, 1, 1, 1]
n_max = len(knots_v)
s_max = n_max - 1 - qmax


x_max = [0 for _ in range(r_max)]
y_max = [0 for _ in range(s_max)]

for i in range(r_max):
    x_max[i] = float(i) +  random.uniform(-0.25, 0.25)

for i in range(s_max):
    y_max[i] = float(i)+  random.uniform(-0.25, 0.25)

z_max = [0 for _ in range(r_max * s_max)]
idx = 0
for i in range(r_max):
    for j in range(s_max):
        idx = i * s_max + j
        z_max[idx] = math.sin(x_max[i]) * math.cos(y_max[j])


weights_max = [0 for _ in range(r_max * s_max)]
wghtsu = [1.0, 1.5, 2.0, 1.5, 1.0, 1.5, 2.0, 1.5, 1.0, 1.5, 2.0, 1.5]
wghtsv  = [1.0, 1.5, 2.0, 1.5, 1.0, 1.5, 2.0, 1.5, 1.0, 1.5, 2.0, 1.5, 1.0]
ind = 0
for i in range(r_max):
    for j in range(s_max):
        weights_max[ind] = wghtsu[i] * wghtsv[j]
        ind = ind + 1

def getKnotsU(p):

    knots = knots_u[(pmax - p) : (m_max - (pmax - p))]
    return knots

def getKnotsV(q):

    knots = knots_v[(qmax - q) : (n_max - (qmax - q))]
    return knots

def getPoints(p, q, d):

    knotsu = getKnotsU(p)
    knotsv = getKnotsV(q)

    m = len(knotsu)
    r = m - p - 1

    n = len(knotsv)
    s = n - q - 1

    x = x_max[0:r]
    y = y_max[0:s]

    if d == 2:
        P = [[0 for _ in range(2)] for _ in range(r*s)]
    elif d == 3:
        P = [[0 for _ in range(3)] for _ in range(r*s)]

    for i in range(r):
        for j in range(s):
            index = i + j * r # NOTE that this is column-major for topohedral

            if d == 2:
                P[index] = [x[i], y[j]]
            else:
                P[index] = [x[i], y[j], z_max[i * s + j]]

    return P

def getPoints2(p, q, d):

    knotsu = getKnotsU(p)
    knotsv = getKnotsV(q)

    m = len(knotsu)
    r = m - p - 1

    n = len(knotsv)
    s = n - q - 1

    x = x_max[0:r]
    y = y_max[0:s]

    if d == 2:
        P = [[0 for _ in range(2)] for _ in range(r*s)]
    elif d == 3:
        P = [[0 for _ in range(3)] for _ in range(r*s)]

    for i in range(r):
        for j in range(s):
            index = i * s + j # NOTE this is the other way round as geomdl is row-major

            if d == 2:
                P[index] = [x[i], y[j]]
            else:
                P[index] = [x[i], y[j], z_max[index]]

    return P

def getWeights(p, q, tr):

    knotsu = getKnotsU(p)
    knotsv = getKnotsV(q)

    m = len(knotsu)
    r = m - p - 1

    n = len(knotsv)
    s = n - q - 1

    nw = s * r

    weights = weights_max[0:nw]

    if tr: 
        tmp_w = np.zeros((r, s))
        for i in range(r):
            for j in range(s):
                tmp_w[i, j] = weights[i*s + j]

        tmp_w_tr = tmp_w.transpose()
        weights = tmp_w_tr.flatten().tolist()
        
    return weights

def getSurface(p, q, d):

    knotsu = getKnotsU(p)
    m = len(knotsu)
    r = m - p - 1
    knotsv = getKnotsV(q)
    n = len(knotsv)
    s = n - q - 1

    points = getPoints2(p, q, d)
    if d == 2:
        for pnt in points:
            pnt.append(0.0)

    weights = getWeights(p, q, False)

    surf = NURBS.Surface()

    # Set degrees
    surf.degree_u = p
    surf.degree_v = q

    surf.ctrlpts_size_u = r
    surf.ctrlpts_size_v = s
    surf.ctrlpts = points

    surf.weights = weights

    # Set knot vectors
    surf.knotvector_u = knotsu
    surf.knotvector_v = knotsv

    return surf

def saveParams(data_out: dict):

    nu = 10
    nv = 10
    NP = nu * nv
    u = linspace(0, 1, nu)
    v = linspace(0, 1, nv)
    S = np.array(cartProd(u, v))

    data_out["uv"] = dict()
    data_out["uv"]["description"] = "Set of parameter pairs (u, v), both dimensions linearly spaced from 0 to 1"
    data_out["uv"]["values"] = S.tolist()

def save_orders(data_out: dict):

    data_out["orders"] = dict() 
    data_out["orders"]["description"] = "List of orders (p, q) for the NURBS surfaces"  
    data_out["orders"]["values"] = orders

def saveKnots(data_out: dict):

    for ord in orders:
        knotsu_p = 'knotsu_p%d' % (ord[0])
        knotsv_q = 'knotsv_q%d' % (ord[1])
        data_out[knotsu_p] = dict()
        data_out[knotsu_p]["description"] = f"Knot vector u for NURBS surface of order p={ord[0]}"
        data_out[knotsu_p]["values"] = getKnotsU(ord[0])

        data_out[knotsv_q] = dict()
        data_out[knotsv_q]["description"] = f"Knot vector v for NURBS surface of order q={ord[1]}"
        data_out[knotsv_q]["values"] = getKnotsV(ord[1])

def saveWeights(data_out: dict):

    for ord in orders:
        weights_p_q = 'weights_p%d_q%d' % (ord[0], ord[1])
        data_out[weights_p_q] = dict()
        data_out[weights_p_q]["description"] = f"Weights for NURBS surface of order (p={ord[0]}, q={ord[1]})"
        data_out[weights_p_q]["values"] = getWeights(ord[0], ord[1], True)

def saveCtrlpts(data_out: dict):

    for d in range(2, 4):
        for ord in orders:

            cpoints_p_q_d = 'cpoints_d%d_p%d_q%d' % (d, ord[0], ord[1])
            cpoints = getPoints(ord[0], ord[1], d)
            data_out[cpoints_p_q_d] = dict()
            data_out[cpoints_p_q_d]["description"] = f"Control points for NURBS surface of order (p={ord[0]}, q={ord[1]}), dimension d={d}"
            data_out[cpoints_p_q_d]["values"] = cpoints

def saveSurfacePoints(data_out: dict):

    nu = 10
    nv = 10
    NP = nu * nv
    u = linspace(0, 1, nu)
    v = linspace(0, 1, nv)
    S = cartProd(u, v)

    for d in range(2, 4):
        for i, ord in enumerate(orders):
            p = ord[0]
            q = ord[1]
            surface = getSurface(p, q, d)
            P = surface.evaluate_list(S)

            if d == 2:
                for pnt in P:
                    pnt.pop()

            dataset = "points_d%i_p%i_q%i" % (d, p, q)
            data_out[dataset] = dict()
            data_out[dataset]["description"] = f"Surface points for NURBS surface of order (p={p}, q={q}), dimension d={d}"
            data_out[dataset]["values"] = P

            # print("------------------------------------- {} {} {}".format(d, p, q))
            # print(P)

def saveDerivatives(data_out: dict):

    nu = 10
    nv = 10
    num_points = nu * nv
    u = linspace(0, 1, nu)
    v = linspace(0, 1, nv)
    S = np.array(cartProd(u, v))

    for d in range(2, 4):
        for ord in orders:
            p = ord[0]
            q = ord[1]
            max_deriv = 4
            surf = getSurface(p, q, d)

            derivs = [[0.0 for _ in range(3)] for _ in range(((max_deriv + 1)**2) * num_points)]

            for j, Sj in enumerate(S):
                u = Sj[0]
                v = Sj[1]
                SKL = surf.derivatives(u, v, max_deriv)

                start = j * (max_deriv+1) * (max_deriv+1)

                for ii in range(max_deriv+1):
                    for jj in range(max_deriv+1):

                        index = start + ii * (max_deriv+1) + jj
                        der_j = SKL[jj][ii]
                        if d == 2:
                            der_j.pop()

                        derivs[index] = der_j
            
            dataset = "ders_d%i_p%i_q%i" % (d, p, q)    
            data_out[dataset] = dict()
            data_out[dataset]["description"] = f"Surface derivatives for NURBS surface of order (p={p}, q={q}), dimension d={d}"
            data_out[dataset]["values"] = derivs
    




def saveTangents():

    f = h5py.File(file_name, 'a')
    grp = f.create_group('surface_tangents')

    nu = 10
    nv = 10
    NP = nu * nv
    u = linspace(0, 1, nu)
    v = linspace(0, 1, nv)
    S = np.array(cartProd(u, v))


    for i, ord in enumerate(orders):

        p = ord[0]
        q = ord[1]
        surf = getSurface(p, q, 3)

        tani = np.zeros((3, 3))
        tangents = np.zeros((NP * 2, 3))

        dset = grp.create_dataset('tangents%i' % i,
                                    (3, NP * 2),
                                    np.double)

        for j, Sj in enumerate(S):

            tmp = operations.tangent(surf, Sj, normalize = False)
            tangents[2*j, :] = tmp[1]
            tangents[2*j+1, :] = tmp[2]

        dset[:,:] = tangents.transpose()

def saveNormals():

    f = h5py.File(file_name, 'a')
    grp = f.create_group('surface_normals')

    nu = 10
    nv = 10
    NP = nu * nv
    u = linspace(0, 1, nu)
    v = linspace(0, 1, nv)
    S = np.array(cartProd(u, v))


    for i, ord in enumerate(orders):

        p = ord[0]
        q = ord[1]
        surf = getSurface(p, q, 3)

        normi = np.zeros((2, 3))
        normals = np.zeros((NP, 3))

        dset = grp.create_dataset('normals%i' % i,
                                    (3, NP),
                                    np.double)

        for j, Sj in enumerate(S):

            tmp = operations.normal(surf, Sj, normalize = False)
            normals[j, :] = tmp[1]

        dset[:,:] = normals.transpose()


def insertKnotU():

    f = h5py.File(file_name, 'a')
    grp = f.create_group('knot_insertion')
    p = orders[4][0]
    q = orders[4][1]

    # ......................................................... insertion1
    surf1 = getSurface(p, q, 3)
    u1 = 0.05
    r1 = 5
    surf1.insert_knot(u1, None, num_u = r1, num_v = 0)

    knots1 = np.array(surf1.knotvector_u)
    points1 = np.array(surf1.ctrlpts)
    pointsw1 = np.array(surf1.ctrlptsw)
    weights1 = np.array(surf1.weights)

    grp1 = grp.create_group('insertionu1')

    dset = grp1.create_dataset('knot', (1,1), dtype = np.double)
    dset[:,:] = u1

    dset = grp1.create_dataset('r', (1,1), np.int)
    dset[:,:] = r1

    dset = grp1.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()

    dset = grp1.create_dataset('points', (points1.shape[1], points1.shape[0]),
                                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp1.create_dataset('pointsw', (pointsw1.shape[1], pointsw1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = pointsw1.transpose()

    dset = grp1.create_dataset('weights', (1, weights1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = weights1.transpose()

    # ......................................................... insertion2
    surf1 = getSurface(p, q, 3)
    u1 = 0.5
    r1 = 4
    surf1.insert_knot(u1, None, num_u = r1, num_v = 0)

    knots1 = np.array(surf1.knotvector_u)
    points1 = np.array(surf1.ctrlpts)
    pointsw1 = np.array(surf1.ctrlptsw)
    weights1 = np.array(surf1.weights)

    grp1 = grp.create_group('insertionu2')

    dset = grp1.create_dataset('knot', (1,1), dtype = np.double)
    dset[:,:] = u1

    dset = grp1.create_dataset('r', (1,1), np.int)
    dset[:,:] = r1

    dset = grp1.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()

    dset = grp1.create_dataset('points', (points1.shape[1], points1.shape[0]),
                                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp1.create_dataset('pointsw', (pointsw1.shape[1], pointsw1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = pointsw1.transpose()

    dset = grp1.create_dataset('weights', (1, weights1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = weights1.transpose()


def insertKnotV():

    f = h5py.File(file_name, 'a')
    grp = f['knot_insertion']
    p = orders[4][0]
    q = orders[4][1]

    # ......................................................... insertion1
    surf1 = getSurface(p, q, 3)
    v1 = 0.05
    r1 = 6
    surf1.insert_knot(None, v1, num_u = 0, num_v = r1)

    knots1 = np.array(surf1.knotvector_v)
    points1 = np.array(surf1.ctrlpts)
    pointsw1 = np.array(surf1.ctrlptsw)
    weights1 = np.array(surf1.weights)

    grp1 = grp.create_group('insertionv1')

    dset = grp1.create_dataset('knot', (1,1), dtype = np.double)
    dset[:,:] = v1

    dset = grp1.create_dataset('r', (1,1), np.int)
    dset[:,:] = r1

    dset = grp1.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()

    dset = grp1.create_dataset('points', (points1.shape[1], points1.shape[0]),
                                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp1.create_dataset('pointsw', (pointsw1.shape[1], pointsw1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = pointsw1.transpose()

    dset = grp1.create_dataset('weights', (1, weights1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = weights1.transpose()

    # ......................................................... insertion2
    surf1 = getSurface(p, q, 3)
    v1 = 0.5
    r1 = 5
    surf1.insert_knot(None, v1, num_u = 0, num_v = r1)

    knots1 = np.array(surf1.knotvector_v)
    points1 = np.array(surf1.ctrlpts)
    pointsw1 = np.array(surf1.ctrlptsw)
    weights1 = np.array(surf1.weights)

    grp1 = grp.create_group('insertionv2')
    dset = grp1.create_dataset('knot', (1,1), dtype = np.double)
    dset[:,:] = v1

    dset = grp1.create_dataset('r', (1,1), np.int)
    dset[:,:] = r1

    dset = grp1.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()

    dset = grp1.create_dataset('points', (points1.shape[1], points1.shape[0]),
                                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp1.create_dataset('pointsw', (pointsw1.shape[1], pointsw1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = pointsw1.transpose()

    dset = grp1.create_dataset('weights', (1, weights1.shape[0]),
                                                dtype = np.double)

    dset[:,:] = weights1.transpose()



def splitSurfaceU():

    f = h5py.File(file_name, 'a')
    grp = f.create_group('surface_splitting')
    p = orders[4][0]
    q = orders[4][1]

    # ......................................................... insertion1
    surf1 = getSurface(p, q, 3)
    surfaces = operations.split_surface_u(surf1, 0.5)

    knots1 = np.array(surfaces[0].knotvector_u)
    points1 = np.array(surfaces[0].ctrlpts)
    pointsw1 = np.array(surfaces[0].ctrlptsw)
    weights1 = np.array(surfaces[0].weights)

    knots2 = np.array(surfaces[1].knotvector_u)
    points2 = np.array(surfaces[1].ctrlpts)
    pointsw2 = np.array(surfaces[1].ctrlptsw)
    weights2 = np.array(surfaces[1].weights)

    grp0 = grp.create_group('splitu1/surface0')
    grp1 = grp.create_group('splitu1/surface1')

    dset = grp0.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()


    dset = grp0.create_dataset('points',
                                (points1.shape[1], points1.shape[0]),
                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp0.create_dataset('pointsw',
                                (pointsw1.shape[1], pointsw1.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw1.transpose()

    dset = grp0.create_dataset('weights',
                                (1, weights1.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights1.transpose()


    # -------
    dset = grp1.create_dataset('knots',
                                (1, knots2.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots2.transpose()


    dset = grp1.create_dataset('points',
                                (points2.shape[1], points2.shape[0]),
                                dtype = np.double)
    dset[:,:] = points2.transpose()

    dset = grp1.create_dataset('pointsw',
                                (pointsw2.shape[1], pointsw2.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw2.transpose()

    dset = grp1.create_dataset('weights',
                                (1, weights2.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights2.transpose()


    # ......................................................... insertion2
    surf1 = getSurface(p, q, 3)
    surfaces = operations.split_surface_u(surf1, 0.05)

    knots1 = np.array(surfaces[0].knotvector_u)
    points1 = np.array(surfaces[0].ctrlpts)
    pointsw1 = np.array(surfaces[0].ctrlptsw)
    weights1 = np.array(surfaces[0].weights)

    knots2 = np.array(surfaces[1].knotvector_u)
    points2 = np.array(surfaces[1].ctrlpts)
    pointsw2 = np.array(surfaces[1].ctrlptsw)
    weights2 = np.array(surfaces[1].weights)

    grp0 = grp.create_group('splitu2/surface0')
    grp1 = grp.create_group('splitu2/surface1')

    dset = grp0.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()


    dset = grp0.create_dataset('points',
                                (points1.shape[1], points1.shape[0]),
                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp0.create_dataset('pointsw',
                                (pointsw1.shape[1], pointsw1.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw1.transpose()

    dset = grp0.create_dataset('weights',
                                (1, weights1.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights1.transpose()


    # -------
    dset = grp1.create_dataset('knots',
                                (1, knots2.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots2.transpose()


    dset = grp1.create_dataset('points',
                                (points2.shape[1], points2.shape[0]),
                                dtype = np.double)
    dset[:,:] = points2.transpose()

    dset = grp1.create_dataset('pointsw',
                                (pointsw2.shape[1], pointsw2.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw2.transpose()

    dset = grp1.create_dataset('weights',
                                (1, weights2.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights2.transpose()





def splitSurfaceV():

    f = h5py.File(file_name, 'a')
    grp = f['surface_splitting']
    p = orders[4][0]
    q = orders[4][1]

    # ......................................................... insertion1
    surf1 = getSurface(p, q, 3)
    surfaces = operations.split_surface_v(surf1, 0.5)

    knots1 = np.array(surfaces[0].knotvector_v)
    points1 = np.array(surfaces[0].ctrlpts)
    pointsw1 = np.array(surfaces[0].ctrlptsw)
    weights1 = np.array(surfaces[0].weights)

    knots2 = np.array(surfaces[1].knotvector_v)
    points2 = np.array(surfaces[1].ctrlpts)
    pointsw2 = np.array(surfaces[1].ctrlptsw)
    weights2 = np.array(surfaces[1].weights)

    grp0 = grp.create_group('splitv1/surface0')
    grp1 = grp.create_group('splitv1/surface1')

    dset = grp0.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()


    dset = grp0.create_dataset('points',
                                (points1.shape[1], points1.shape[0]),
                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp0.create_dataset('pointsw',
                                (pointsw1.shape[1], pointsw1.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw1.transpose()

    dset = grp0.create_dataset('weights',
                                (1, weights1.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights1.transpose()


    # -------
    dset = grp1.create_dataset('knots',
                                (1, knots2.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots2.transpose()


    dset = grp1.create_dataset('points',
                                (points2.shape[1], points2.shape[0]),
                                dtype = np.double)
    dset[:,:] = points2.transpose()

    dset = grp1.create_dataset('pointsw',
                                (pointsw2.shape[1], pointsw2.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw2.transpose()

    dset = grp1.create_dataset('weights',
                                (1, weights2.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights2.transpose()


    # ......................................................... insertion2
    surf1 = getSurface(p, q, 3)
    surfaces = operations.split_surface_v(surf1, 0.05)

    knots1 = np.array(surfaces[0].knotvector_v)
    points1 = np.array(surfaces[0].ctrlpts)
    pointsw1 = np.array(surfaces[0].ctrlptsw)
    weights1 = np.array(surfaces[0].weights)

    knots2 = np.array(surfaces[1].knotvector_v)
    points2 = np.array(surfaces[1].ctrlpts)
    pointsw2 = np.array(surfaces[1].ctrlptsw)
    weights2 = np.array(surfaces[1].weights)

    grp0 = grp.create_group('splitv2/surface0')
    grp1 = grp.create_group('splitv2/surface1')

    dset = grp0.create_dataset('knots',
                                (1, knots1.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots1.transpose()


    dset = grp0.create_dataset('points',
                                (points1.shape[1], points1.shape[0]),
                                dtype = np.double)
    dset[:,:] = points1.transpose()

    dset = grp0.create_dataset('pointsw',
                                (pointsw1.shape[1], pointsw1.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw1.transpose()

    dset = grp0.create_dataset('weights',
                                (1, weights1.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights1.transpose()


    # -------
    dset = grp1.create_dataset('knots',
                                (1, knots2.shape[0]),
                                dtype = np.double)
    dset[:,:] = knots2.transpose()


    dset = grp1.create_dataset('points',
                                (points2.shape[1], points2.shape[0]),
                                dtype = np.double)
    dset[:,:] = points2.transpose()

    dset = grp1.create_dataset('pointsw',
                                (pointsw2.shape[1], pointsw2.shape[0]),
                                dtype = np.double)
    dset[:,:] = pointsw2.transpose()

    dset = grp1.create_dataset('weights',
                                (1, weights2.shape[0]),
                                dtype = np.double)
    dset[:,:] = weights2.transpose()



def saveIntegration():

    p = 2
    q = 2
    knotsu = [0, 0, 0, 0.34, 0.57, 0.86, 1,1,1]
    knotsv = [0, 0, 0, 0.124, 0.45, 0.73, 1,1,1]
    r = len(knotsu) - (p + 1)
    s = len(knotsv) - (q + 1)
    points = [[0 for i in range(3)] for i in range(r*s)]
    for i in range(r):
        for j in range(s):
            index = i*s + j
            points[index] = [i, j, math.sin(i)*math.cos(j)]

    weights = [1 for i in range(r*s)]


    surf = NURBS.Surface()
    surf.degree_u = p
    surf.degree_v = q

    surf.ctrlpts_size_u = r
    surf.ctrlpts_size_v = s
    surf.ctrlpts = points

    surf.weights = weights

    surf.knotvector_u = knotsu
    surf.knotvector_v = knotsv


    knotsu1 = np.array(surf.knotvector_u)
    knotsv1 = np.array(surf.knotvector_v)
    points1 = np.array(surf.ctrlpts)
    weights1 = np.array(surf.weights)

    def jacobian(u,v):
        s = [u,v]
        tmp = operations.tangent(surf, s, normalize = False)
        tanu = tmp[1]
        tanv = tmp[2]
        tangents = np.array([tanu, tanv])
        jac_mat = np.matmul(tangents, tangents.transpose())
        jac = math.sqrt(np.linalg.det(jac_mat))
        return jac


    def fcn3d(x, y, z):
        val = x * y * z * (math.exp(y)/500)
        return val

    def integrand(v,u):
        s = [u,v]
        X = surf.evaluate_single(s)
        x = X[0]
        y = X[1]
        z = X[2]
        jac = jacobian(u,v)
        f = jac * fcn3d(x,y,z)
        return f


    def ylow(x):
        return 0

    def yhigh(x):
        return 1

    If, err = sci.dblquad(integrand, 0, 1, ylow, yhigh, epsabs = 1e-3, epsrel = 1e-3)
    print(If)

    f = h5py.File(file_name, 'a')
    grp = f.create_group('integration')

    dset = grp.create_dataset('p', (1,0), np.int)
    dset[0] = p

    dset = grp.create_dataset('q', (1,0), np.int)
    dset[0] = q

    dset = grp.create_dataset('knotsu', (1, knotsu1.shape[0]), np.double)
    dset[:,:] = knotsu1.transpose()

    dset = grp.create_dataset('knotsv', (1, knotsv1.shape[0]), np.double)
    dset[:,:] = knotsv1.transpose()

    dset = grp.create_dataset('points', (points1.shape[1], points1.shape[0]), np.double)
    dset[:,:] = points1.transpose()

    dset = grp.create_dataset('weights', (1, weights1.shape[0]), np.double)
    dset[:,:] = weights1.transpose()

    dset = grp.create_dataset('integral', (1,), np.double)
    dset[0] = If




def cartProd(x, y):

    n = len(x)
    m = len(y)

    N = n * m
    P = [[0 for _ in range(2)] for _ in range(N)]

    ii = 0
    for i in range(n):
        for j in range(m):

            P[ii] = [x[i], y[j]]
            ii = ii + 1

    return P

def linspace(a, b, n):

    dx = (b - a) / (n-1)
    x = [0 for _ in range(n)]
    x[0] = a
    x[n-1] = b
    for i in range(1, n-1):
        x[i] = x[i-1] + dx

    return x




def main():


    data_out = dict()
    saveParams(data_out)
    saveKnots(data_out)
    saveWeights(data_out)
    saveCtrlpts(data_out)
    saveSurfacePoints(data_out)
    saveDerivatives(data_out)

    with open(file_name, 'w') as f: 
        json.dump(data_out, f, indent=4)


    surf1 = getSurface(1, 2, 3)

    for i in range(surf1.ctrlpts_size_u):
        for j in range(surf1.ctrlpts_size_v):
            print('[{}][{}] = {}'.format(i, j, surf1.ctrlpts2d[i][j]))

    # saveSurfacePoints()
    # saveDerivatives()
    # saveTangents()
    # saveNormals()
    # insertKnotU()
    # insertKnotV()
    # splitSurfaceU()
    # splitSurfaceV()
    # saveIntegration()




if __name__ == '__main__':
    main()
