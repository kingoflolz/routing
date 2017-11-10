import keras
from keras.models import Sequential
from keras.layers import Dense, Dropout, Activation
from keras import metrics

from sklearn.metrics import confusion_matrix

import numpy

import random
import functools

data = open("data_core").readlines()

data_in = []
labels = []

a = [1.0709195,0.65015,0.0001,0.565759,0.0001,2.608484,1.5629014,1.5335436,0.059993483,0.39511952]
b = [0.39104798,1.2509017,1.9639459,0.6434256,2.2688494,-0.06174837,-0.102137364,0.62045085,0.8320958,0.8513484]

random.shuffle(data)

for i in data:
    labels.append(int(i.split(", ")[0]))
    k = i.split("outgoing_vec: Matrix { data: [")[1].split("]")[0].split(", ")
    o = []
    for j, k in zip(k, b):
        o.append(float(j))
        #o.append(float(j)*float(k))
    i = i.split("incoming_vec: Matrix { data: [")[1].split("]")[0].split(", ")
    for j, k in zip(i, a):
        o.append(float(j))
        #o.append(float(j)*float(k))
    data_in.append(o)

seq_lab = {}
for i in labels:
    if i not in seq_lab:
        seq_lab[i] = len(seq_lab)

labels_out = []
for i in labels:
    labels_out.append(seq_lab[i])


model = Sequential([
    Dense(20, input_shape=(20,)),
    Activation('relu'),
    Dense(20),
    Activation('relu'),
    Dense(20),
    Activation('relu'),
    Dense(max(labels_out)+1),
    Activation('softmax')
])

top2_acc = functools.partial(keras.metrics.top_k_categorical_accuracy, k=2)

top2_acc.__name__ = 'top2_acc'

model.compile(optimizer=keras.optimizers.Adam(lr=0.001, beta_1=0.9, beta_2=0.999, epsilon=1e-08, decay=0.0),
              loss='categorical_crossentropy',
              metrics=['accuracy', top2_acc])

model.fit(data_in[:1000], keras.utils.to_categorical(labels_out[:1000], num_classes=max(labels_out)+1), epochs=100, batch_size=16)
print(len(labels_out))
print(model.evaluate(x=data_in[1000:], y=keras.utils.to_categorical(labels_out[1000:], num_classes=max(labels_out)+1)))

numpy.savetxt("foo.csv", model.predict(data_in[1000:]), delimiter=",")

