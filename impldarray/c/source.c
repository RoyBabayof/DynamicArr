#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

typedef struct{
    size_t capacity;
    size_t type_size;
} Metadata;

typedef struct{
    Metadata metadata;
    void *data;
} DynamicArray;

DynamicArray dynamicarray(size_t capacity, size_t type_size){
    Metadata metadata =  {capacity, type_size};
    DynamicArray array = {metadata, (void*) malloc(capacity * type_size)};
    return array;
}

static inline void *slot_at(const DynamicArray *a, size_t index) {
    return (uint8_t *)a->data + index * a->metadata.type_size;
}

void inlargeDynamicArray(int mult, DynamicArray *array){
    size_t old = array->metadata.capacity;
    array->metadata.capacity = array->metadata.capacity* mult;
    array->data = (void *) realloc(array->data, array->metadata.capacity * array->metadata.type_size);
    memset((u_int8_t *)array->data + old * array->metadata.type_size, 0, (array-> metadata.capacity - old) * array->metadata.type_size);

}

void darray_member_safe_initilization(DynamicArray *array, int index, void * value){
    if (index >= array->metadata.capacity){ 
        size_t new_cap = array->metadata.capacity ? array->metadata.capacity : 1;
        while (new_cap <= index) new_cap *= 2;
        inlargeDynamicArray(new_cap, array);
    }
    memcpy((char *)array->data + index * array->metadata.type_size,
     value, array->metadata.type_size);
}

void *darray_get(const DynamicArray *array, size_t index) {
    assert(index < array->metadata.capacity);
    return slot_at(array, index);
}

int main(){
    DynamicArray arr = dynamicarray(4, sizeof(int));
    int integer = 10;
    const int index = 100;
    darray_member_safe_initilization(&arr, index, &integer);
    printf("%d\n" ,*(int *)darray_get(&arr, index));
}

