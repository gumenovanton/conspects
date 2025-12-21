#### COVERING GAPS IN SEQUENCES
// for example i want to do report about sales, where i want te see date and amount of sales on this date
// but if i don't have any sales on a particular day
// how can i understand there were no sales on that day or there is a missing record in the report
// for example query like this
select sale_date, sum(amount) from sales group by sale_date order by sale_date asc;

// to avoid it i can fill my report with entries that will replace the missin entries
// like this

select
    all_dates.sale_date::date, coalesce(total_amount, 0) as total_amount // calesce replase all false values to second parameter(here 0)
from
    gonorate_series('2024-01-01'::date, '2024-01-31'::date, '1 day') as all_dates(sale_date) //all_dates - is a table alias, sale_date - is a field alias
left join(
    select sum(amount) as total_amount from sales group by sale_date
) as sales on sales.sale_date = all_dates.sale_date;

// sale_date        total_amount
// 2024-01-01       1233.33
// ...
// 2024-01-14       0
