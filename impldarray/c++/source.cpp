#include <iostream>
#include <vector>
#include <memory>
#include <initializer_list>
#define DEFAULT_VECTOR_SIZE 1

template <typename T>
class vector{
private:
    size_t capacity;
    size_t size;
    std::unique_ptr<T[]> data;
public:
    // We can use capacity when initlizing data since its stated as the first property
    explicit vector(): capacity(DEFAULT_VECTOR_SIZE), size(0), data(new T[capacity])
    {}
    vector(std::initializer_list<T> l): size(l.size())
    {
        std::move(l.begin(),l.end())
    }
};

int main(){
    std::vector<int> v = {1,2,3,4};
    std::allocator<int> a;

}