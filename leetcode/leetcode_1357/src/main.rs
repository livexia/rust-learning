/*
超市里正在举行打折活动，每隔 n 个顾客会得到 discount 的折扣。

超市里有一些商品，第 i 种商品为 productsvec![i] 且每件单品的价格为 pricesvec![i] 。

结账系统会统计顾客的数目，每隔 n 个顾客结账时，该顾客的账单都会打折，折扣为 discount （也就是如果原本账单为 x ，那么实际金额会变成 x - (discount * x) / 100 ），然后系统会重新开始计数。

顾客会购买一些商品， productvec![i] 是顾客购买的第 i 种商品， amountvec![i] 是对应的购买该种商品的数目。

请你实现 Cashier 类：

Cashier(int n, int discount, intvec![] products, intvec![] prices) 初始化实例对象，参数分别为打折频率 n ，折扣大小 discount ，超市里的商品列表 products 和它们的价格 prices 。
double get_bill(intvec![] product, intvec![] amount) 返回账单的实际金额（如果有打折，请返回打折后的结果）。返回结果与标准答案误差在 10^-5 以内都视为正确结果。
 

示例 1：

输入
vec!["Cashier","get_bill","get_bill","get_bill","get_bill","get_bill","get_bill","get_bill"]
vec![vec![3,50,vec![1,2,3,4,5,6,7],vec![100,200,300,400,300,200,100]],vec![vec![1,2],vec![1,2]],vec![vec![3,7],vec![10,10]],vec![vec![1,2,3,4,5,6,7],vec![1,1,1,1,1,1,1]],vec![vec![4],vec![10]],vec![vec![7,3],vec![10,10]],vec![vec![7,5,3,1,6,4,2],vec![10,10,10,9,9,9,7]],vec![vec![2,3,5],vec![5,3,2]]]
输出
vec![null,500.0,4000.0,800.0,4000.0,4000.0,7350.0,2500.0]
解释
Cashier cashier = new Cashier(3,50,vec![1,2,3,4,5,6,7],vec![100,200,300,400,300,200,100]);
cashier.get_bill(vec![1,2],vec![1,2]);                        // 返回 500.0, 账单金额为 = 1 * 100 + 2 * 200 = 500.
cashier.get_bill(vec![3,7],vec![10,10]);                      // 返回 4000.0
cashier.get_bill(vec![1,2,3,4,5,6,7],vec![1,1,1,1,1,1,1]);    // 返回 800.0 ，账单原本为 1600.0 ，但由于该顾客是第三位顾客，他将得到 50% 的折扣，所以实际金额为 1600 - 1600 * (50 / 100) = 800 。
cashier.get_bill(vec![4],vec![10]);                           // 返回 4000.0
cashier.get_bill(vec![7,3],vec![10,10]);                      // 返回 4000.0
cashier.get_bill(vec![7,5,3,1,6,4,2],vec![10,10,10,9,9,9,7]); // 返回 7350.0 ，账单原本为 14700.0 ，但由于系统计数再次达到三，该顾客将得到 50% 的折扣，实际金额为 7350.0 。
cashier.get_bill(vec![2,3,5],vec![5,3,2]);                    // 返回 2500.0
 

提示：

1 <= n <= 10^4
0 <= discount <= 100
1 <= products.length <= 200
1 <= productsvec![i] <= 200
products 列表中 不会 有重复的元素。
prices.length == products.length
1 <= pricesvec![i] <= 1000
1 <= product.length <= products.length
productvec![i] 在 products 出现过。
amount.length == product.length
1 <= amountvec![i] <= 1000
最多有 1000 次对 get_bill 函数的调用。
返回结果与标准答案误差在 10^-5 以内都视为正确结果。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/apply-discount-every-n-orders
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

fn main() {
    let mut cashier = Cashier::new(3,50,vec![1,2,3,4,5,6,7],vec![100,200,300,400,300,200,100]);
    println!("{:?}", cashier);
    cashier.get_bill(vec![1,2],vec![1,2]);                        // 返回 500.0, 账单金额为 = 1 * 100 + 2 * 200 = 500.
    cashier.get_bill(vec![3,7],vec![10,10]);                      // 返回 4000.0
    cashier.get_bill(vec![1,2,3,4,5,6,7],vec![1,1,1,1,1,1,1]);    // 返回 800.0 ，账单原本为 1600.0 ，但由于该顾客是第三位顾客，他将得到 50% 的折扣，所以实际金额为 1600 - 1600 * (50 / 100) = 800 。
    cashier.get_bill(vec![4],vec![10]);                           // 返回 4000.0
    cashier.get_bill(vec![7,3],vec![10,10]);                      // 返回 4000.0
    cashier.get_bill(vec![7,5,3,1,6,4,2],vec![10,10,10,9,9,9,7]); // 返回 7350.0 ，账单原本为 14700.0 ，但由于系统计数再次达到三，该顾客将得到 50% 的折扣，实际金额为 7350.0 。
    cashier.get_bill(vec![2,3,5],vec![5,3,2]);                    // 返回 2500.0
}

#[derive(Debug)]
struct Cashier {
    n: i32,
    count: i32,
    discount: i32,
    prices: Vec<f64>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut price_table: Vec<f64> = Vec::new();
        for _ in 0..(products.iter().max().unwrap() + 1) {
            price_table.push(0.0);
        }
        for i in 0..products.len() {
            price_table[products[i] as usize] = prices[i] as f64;
        }
        Cashier { n, discount, prices: price_table, count: 0 }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.count += 1;
        let mut bill: f64 = 0.0;
        for i in 0..product.len() {
            bill += self.prices[product[i] as usize] * amount[i] as f64;
        }
        if self.count - self.n == 0 {
            bill = bill - bill * (self.discount as f64) / 100.0;
            self.count = 0;
        }
        println!("{}", bill);
        bill
    }
}

/*
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
